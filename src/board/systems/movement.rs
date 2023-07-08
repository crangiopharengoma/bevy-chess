use bevy::app::AppExit;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_mod_picking::{Hover, Selection};

use crate::board;
use crate::board::components::Move;
use crate::board::resources::{
    Graveyard, MoveStack, SelectedPiece, SelectedSquare, SquareMaterials,
};
use crate::board::{MoveMadeEvent, ResetSelectedEvent, Square, Taken};
use crate::pieces::{Piece, PieceType};

pub fn push_move(
    mut stack: ResMut<MoveStack>,
    mut move_events: EventReader<MoveMadeEvent>,
    query: Query<&Piece, Without<Taken>>,
) {
    for move_event in move_events.iter() {
        let pieces: Vec<_> = query.iter().cloned().collect();
        stack.stack.push((*move_event, pieces));
    }
}

pub fn remove_taken_pieces(
    mut exit_event: EventWriter<AppExit>,
    time: Res<Time>,
    mut query: Query<(&Piece, &Taken, &mut Transform)>,
) {
    for (piece, taken, mut transform) in query.iter_mut() {
        if piece.piece_type == PieceType::King {
            println!("{} won! Thanks for playing!", piece.colour);
            exit_event.send(AppExit);
        }

        let direction = taken.grave - transform.translation;

        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds() * 5.0;
        }
    }
}

pub fn colour_moves(
    selected_piece: Res<SelectedPiece>,
    materials: Res<SquareMaterials>,
    move_stack: Res<MoveStack>,
    pieces: Query<&Piece, Without<Taken>>,
    mut squares: Query<(&Square, &mut Handle<StandardMaterial>, &Selection, &Hover)>,
) {
    let moves = if let Some(piece_entity) = selected_piece.entity {
        let piece = pieces.get(piece_entity).expect("unable to retrieve entity");
        let pieces_vec: Vec<_> = pieces.iter().copied().collect();

        let last_move = move_stack.stack.last().map(|(move_event, _)| {
            let last_piece = pieces.get(move_event.piece).unwrap();
            (*last_piece, move_event.origin, move_event.destination)
        });

        piece.legal_moves(&pieces_vec, last_move)
    } else {
        HashSet::new()
    };

    for (square, mut material, selection, hover) in squares.iter_mut() {
        *material = if hover.hovered() {
            materials.hover_colour.clone()
        } else if moves.contains(square) {
            materials.highlight_colour.clone()
        } else if selection.selected() {
            materials.selected_colour.clone()
        } else if hover.hovered() {
            materials.hover_colour.clone()
        } else if square.is_white() {
            materials.white_colour.clone()
        } else {
            materials.black_colour.clone()
        }
    }
}

pub fn make_move(
    mut commands: Commands,
    mut pieces: Query<(Entity, &mut Piece, &Move), Without<Taken>>,
) {
    for (entity, mut piece, movement) in pieces.iter_mut() {
        piece.pos = movement.square;
        piece.has_moved = true;

        commands.entity(entity).remove::<Move>();
    }
}

#[allow(clippy::too_many_arguments)]
pub fn move_piece(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut graveyard: ResMut<Graveyard>,
    move_stack: Res<MoveStack>,
    squares: Query<&Square>,
    pieces: Query<(Entity, &Piece), Without<Taken>>,
    mut reset_selected_event: EventWriter<ResetSelectedEvent>,
    mut move_made_event: EventWriter<MoveMadeEvent>,
) {
    if !selected_square.is_changed() {
        return;
    }

    let square = if let Some(Ok(square)) = selected_square
        .entity
        .map(|square_entity| squares.get(square_entity))
    {
        square
    } else {
        return;
    };

    if let Some(piece_entity) = selected_piece.entity {
        // a piece is selected, so lets move it
        let pieces_vec: Vec<_> = pieces.iter().map(|(_, piece)| *piece).collect();

        let last_move_event = move_stack.stack.last().map(|(event, _)| event);
        let last_move = create_last_move_record(last_move_event, &pieces);
        let (_, moving_piece) = pieces.get(piece_entity).unwrap();

        if moving_piece
            .legal_moves(&pieces_vec, last_move)
            .contains(square)
        {
            let (taken_piece, en_passant) =
                try_get_taken_piece(&pieces, square, piece_entity, last_move_event, &last_move);

            if let Some(entity) = taken_piece {
                commands.entity(entity).insert(Taken {
                    grave: graveyard.next(moving_piece.colour),
                });
            }

            commands
                .entity(piece_entity)
                .insert(Move { square: *square });

            // if castling the rook needs to move too
            if moving_piece.piece_type == PieceType::King
                && (moving_piece.pos.file - square.file).abs() == 2
            {
                move_castling_rook(&mut commands, &pieces, square, moving_piece);
                move_made_event.send(MoveMadeEvent::castling(
                    piece_entity,
                    moving_piece.pos,
                    *square,
                ));
            } else {
                move_made_event.send(MoveMadeEvent::not_castling(
                    piece_entity,
                    moving_piece.pos,
                    *square,
                    taken_piece,
                    en_passant,
                ));
            }
        }

        reset_selected_event.send(ResetSelectedEvent);
    }
}

fn try_get_taken_piece(
    pieces: &Query<(Entity, &Piece), Without<Taken>>,
    square: &Square,
    piece_entity: Entity,
    last_move_event: Option<&MoveMadeEvent>,
    last_move: &Option<(Piece, Square, Square)>,
) -> (Option<Entity>, bool) {
    let (taken_piece, en_passant) = {
        let taken_piece = pieces
            .iter()
            .find(|(_, taken_piece)| taken_piece.pos == *square)
            .map(|(entity, _)| entity);

        if taken_piece.is_none() {
            let taken_piece =
                get_en_passant_piece(pieces, square, piece_entity, last_move_event, last_move);
            (taken_piece, taken_piece.is_some())
        } else {
            (taken_piece, false)
        }
    };
    (taken_piece, en_passant)
}

fn move_castling_rook(
    commands: &mut Commands,
    pieces: &Query<(Entity, &Piece), Without<Taken>>,
    square: &Square,
    moving_piece: &Piece,
) {
    let rook_dest_file = if square.file == board::G_FILE {
        board::F_FILE
    } else {
        board::D_FILE
    };
    let rook_dest_square = Square {
        rank: square.rank,
        file: rook_dest_file,
    };

    let (rook_entity, _) = pieces
        .iter()
        .find(|(_, piece)| {
            piece.piece_type == PieceType::Rook
                && !piece.has_moved
                && piece.colour == moving_piece.colour
                && (piece.pos.file - square.file).abs() < 3
        })
        .unwrap();

    commands.entity(rook_entity).insert(Move {
        square: rook_dest_square,
    });
}

fn get_en_passant_piece(
    pieces: &Query<(Entity, &Piece), Without<Taken>>,
    square: &Square,
    piece_entity: Entity,
    last_move_event: Option<&MoveMadeEvent>,
    last_move: &Option<(Piece, Square, Square)>,
) -> Option<Entity> {
    let last_move_event = last_move_event?;
    if pieces
        .get(piece_entity)
        .unwrap()
        .1
        .may_take_en_passant(square, last_move)
    {
        Some(last_move_event.piece)
    } else {
        None
    }
}

fn create_last_move_record(
    last_move_event: Option<&MoveMadeEvent>,
    pieces: &Query<(Entity, &Piece), Without<Taken>>,
) -> Option<(Piece, Square, Square)> {
    let last_move_event = last_move_event?;
    let (_, last_piece) = pieces.get(last_move_event.piece).unwrap();
    Some((
        *last_piece,
        last_move_event.origin,
        last_move_event.destination,
    ))
}
