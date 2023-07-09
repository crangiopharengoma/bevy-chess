use bevy::prelude::*;
use bevy_mod_picking::{Highlighting, PickableBundle, PickingEvent, Selection, SelectionEvent};

pub use movement::{colour_moves, make_move, move_piece, push_move, remove_taken_pieces};

use crate::board;
use crate::board::components::{Move, Selected, Square, Taken};
use crate::board::events::ResetSelectedEvent;
use crate::board::resources::{DrawReason, MoveHistory, MoveStack, PlayerTurn, SquareMaterials};
use crate::board::{
    GameStatus, MoveMadeEvent, MoveType, Promote, PromotionOutcome, SelectPromotionOutcome,
};
use crate::pieces::{Piece, PieceColour, PieceType};

mod movement;

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    square_materials: Res<SquareMaterials>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.0,
        subdivisions: 0,
    }));

    for rank in board::RANK_1..=board::RANK_8 {
        for file in board::A_FILE..=board::H_FILE {
            let square = Square { rank, file };
            let initial_material = if square.is_white() {
                square_materials.white_colour.clone()
            } else {
                square_materials.black_colour.clone()
            };
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: initial_material.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        rank as f32,
                        0.0,
                        file as f32,
                    )),
                    ..Default::default()
                },
                PickableBundle::default(),
                Highlighting {
                    initial: initial_material.clone(),
                    hovered: Some(square_materials.hover_colour.clone()),
                    pressed: None,
                    selected: Some(square_materials.selected_colour.clone()),
                },
                Square { rank, file },
            ));
        }
    }
}

/// Consumes events from Bevy_Mod_Picking and adds the `Selected` marker component when an element
/// is selected, and removes it when it is deselected
pub fn select_square(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    // pieces: Query<(Entity, &Piece), Without<Taken>>,
    // square: Query<&Square>,
    // mut selected_square: ResMut<SelectedSquare>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(event) = event {
            match event {
                SelectionEvent::JustSelected(entity) => {
                    // println!("New square selected {entity:?}");
                    // selected_square.entity = Some(*entity);
                    // let selected = square.get(*entity);
                    // println!("{selected:?} was selected");
                    commands.entity(*entity).insert(Selected);
                }
                SelectionEvent::JustDeselected(entity) => {
                    // JustDeselected fires when the user is unselecting the current square or when
                    // they select a new square (the previously selected square is unselected. So we
                    // should only clear the SelectedSquare resource when it is the same as the
                    // deselected entity
                    // let deselected = square.get(*entity);
                    // println!("{deselected:?} was selected");
                    commands.entity(*entity).remove::<Selected>();
                    // in case a piece has been selected
                    // for (entity, _) in pieces.iter() {
                    //     commands.entity(entity).remove::<Selected>();
                    // }
                    // if selected_square.entity == Some(*entity) {
                    //     selected_square.entity = None;
                    // }
                }
            }
        }
    }
}

/// If a selected square contains a piece then give that piece the `Selected` marker trait also
pub fn select_piece(
    mut commands: Commands,
    // selected_square: Res<SelectedSquare>,
    // mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    squares: Query<(&Square, &Selected)>,
    pieces: Query<(Entity, &Piece, Option<&Selected>), Without<Taken>>,
) {
    for (square, _) in squares.iter() {
        for (entity, piece, selected) in pieces.iter() {
            if piece.pos.eq(square) && piece.colour == turn.0 && selected.is_none() {
                println!("{piece:?} was selected");
                commands.entity(entity).insert(Selected);
            } else if selected.is_some() && piece.colour == turn.0 && piece.pos.ne(square) {
                println!("{piece:?} was deselected");
                commands.entity(entity).remove::<Selected>();
            }
        }
    }

    // if !selected_square.is_changed() {
    //     return;
    // }
    //
    // let square = if let Some(Ok(square)) = selected_square
    //     .entity
    //     .map(|square_entity| squares.get(square_entity))
    // {
    //     square
    // } else {
    //     return;
    // };
    //
    // if selected_piece.entity.is_none() {
    //     selected_piece.entity = pieces
    //         .iter()
    //         .find(|(_, piece)| piece.pos == *square && piece.colour == turn.0)
    //         .map(|(entity, _)| entity);
    // }
}

pub fn reset_selected(
    mut commands: Commands,
    mut event_reader: EventReader<ResetSelectedEvent>,
    // mut selected_square: ResMut<SelectedSquare>,
    // mut selected_piece: ResMut<SelectedPiece>,
    mut selected_squares: Query<(Entity, &Square, &Selected, &mut Selection)>,
    selected_pieces: Query<(Entity, &Piece, &Selected)>,
    // mut query: Query<&mut Selection>,
) {
    for _ in event_reader.iter() {
        // if let Some(square) = selected_square.entity {
        //     if let Ok(mut selection) = query.get_mut(square) {
        //         selection.set_selected(false)
        //     }
        // }

        for (entity, _, _, mut selection) in selected_squares.iter_mut() {
            selection.set_selected(false);
            commands.entity(entity).remove::<Selected>();
        }

        for (entity, _, _) in selected_pieces.iter() {
            commands.entity(entity).remove::<Selected>();
        }

        // selected_square.entity = None;
        // selected_piece.entity = None;
    }
}

pub fn select_promotion(
    mut event_writer: EventWriter<SelectPromotionOutcome>,
    pieces: Query<(Entity, &Piece, &Move), Without<Taken>>,
) {
    for (entity, piece, movement) in pieces.iter() {
        if piece.piece_type == PieceType::Pawn
            && (movement.square.rank == board::RANK_1 || movement.square.rank == board::RANK_8)
        {
            event_writer.send(SelectPromotionOutcome { entity });
        }
    }
}

pub fn promote_piece(
    mut commands: Commands,
    mut move_history: ResMut<MoveHistory>,
    mut event_reader: EventReader<PromotionOutcome>,
) {
    for event in event_reader.iter() {
        let promote = Promote {
            to: event.piece_type,
        };

        commands.entity(event.entity).insert(promote);
        move_history
            .0
            .last_mut()
            .unwrap()
            .push_str(&format!("={}", event.piece_type.notation_letter()));
    }
}

pub fn update_status(
    mut last_action: Local<i32>,
    mut move_number: Local<u32>,
    move_stack: Res<MoveStack>,
    mut move_history: ResMut<MoveHistory>,
    mut turn: ResMut<PlayerTurn>,
    mut game_status: ResMut<GameStatus>,
    pieces: Query<&Piece>,
) {
    if move_stack.stack.is_empty() || !move_stack.is_changed() {
        return;
    }

    let pieces_vec: Vec<_> = pieces.iter().copied().collect();
    let (last_move_event, _) = move_stack.stack.last().unwrap();
    let moving_piece = pieces.get(last_move_event.piece).unwrap();
    let last_move_record = Some((
        *moving_piece,
        last_move_event.origin,
        last_move_event.destination,
    ));

    if moving_piece.piece_type == PieceType::Pawn || last_move_event.is_take() {
        *last_action = 0
    } else {
        *last_action += 1
    }

    let has_moves = player_has_moves(
        turn.0.opponent(),
        &pieces_vec,
        &pieces_vec,
        last_move_record,
    );
    let check = is_in_check(
        turn.0.opponent(),
        &pieces_vec,
        &pieces_vec,
        last_move_record,
    );

    *game_status = if check && !has_moves {
        GameStatus::Checkmate
    } else if *last_action == 50 {
        GameStatus::Draw(DrawReason::FiftyMoveRule)
    } else if check & has_moves {
        turn.change();
        GameStatus::Check
    } else if !check && !has_moves {
        GameStatus::Draw(DrawReason::Stalemate)
    } else {
        // TODO other draw conditions
        turn.change();
        GameStatus::OnGoing
    };

    let destination = last_move_event.destination;

    // let pieces: Vec<_> = pieces.iter().map(|(piece, _)| *piece).collect();

    if moving_piece.colour == PieceColour::White {
        *move_number += 1;
        let move_annotation = generate_move_annotation(
            &format!("{}. ", *move_number),
            last_move_event,
            last_move_record,
            moving_piece,
            &pieces_vec,
            &destination,
            game_status.as_ref(),
        );
        move_history.0.push(move_annotation);
    } else {
        let current = move_history.0.last_mut().unwrap();
        *current = generate_move_annotation(
            current,
            last_move_event,
            last_move_record,
            moving_piece,
            &pieces_vec,
            &destination,
            game_status.as_ref(),
        );
    }
}

fn generate_move_annotation(
    prefix: &str,
    event: &MoveMadeEvent,
    last_move: Option<(Piece, Square, Square)>,
    moving_piece: &Piece,
    pieces: &[Piece],
    destination: &Square,
    status: &GameStatus,
) -> String {
    let ambiguous_pieces: Vec<_> = pieces
        .iter()
        .filter(|piece| {
            piece.colour == moving_piece.colour
                && piece.piece_type == moving_piece.piece_type
                && piece.legal_moves(pieces, last_move).contains(destination)
                && piece.pos != moving_piece.pos
        })
        .collect();

    let file_ambiguous = ambiguous_pieces
        .iter()
        .any(|piece| piece.pos.file == moving_piece.pos.file);
    let rank_ambiguous = ambiguous_pieces
        .iter()
        .any(|piece| piece.pos.rank == moving_piece.pos.rank);

    let disambiguator = if file_ambiguous {
        if rank_ambiguous {
            moving_piece.pos.to_string()
        } else {
            moving_piece.pos.rank_annotation()
        }
    } else if !ambiguous_pieces.is_empty() {
        moving_piece.pos.file_annotation()
    } else {
        String::new()
    };

    let status = match status {
        GameStatus::Check => "!",
        GameStatus::Checkmate => "#",
        _ => "",
    };

    match event.move_type {
        MoveType::Take(_) | MoveType::TakeEnPassant(_) => {
            let piece_letter = if moving_piece.piece_type == PieceType::Pawn {
                moving_piece
                    .pos
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
            } else {
                moving_piece.piece_type.notation_letter()
            };
            format!(
                "{prefix} {piece_letter}{disambiguator}x{destination}{status}",
                // piece.piece_type.notation_letter()
            )
        }
        MoveType::Castle => {
            if destination.file == board::G_FILE {
                format!("{prefix} 0-0{status}")
            } else {
                format!("{prefix} 0-0-0{status}")
            }
        }
        MoveType::Move => {
            format!(
                "{prefix} {}{disambiguator}{destination}{status}",
                moving_piece.piece_type.notation_letter()
            )
        }
    }
}

fn is_in_check(
    player_colour: PieceColour,
    pieces: &[Piece],
    pieces_vec: &[Piece],
    last_move: Option<(Piece, Square, Square)>,
) -> bool {
    let own_king = pieces_vec
        .iter()
        .find(|piece| piece.colour == player_colour && piece.piece_type == PieceType::King)
        .unwrap();

    // FIXME a pinned piece still gives check even though it's not considered a legal move
    pieces
        .iter()
        .filter(|piece| piece.colour == player_colour.opponent())
        .any(|piece| {
            piece
                .legal_moves(pieces_vec, last_move)
                .contains(&own_king.pos)
        })
}

fn player_has_moves(
    player_colour: PieceColour,
    pieces: &[Piece],
    pieces_vec: &[Piece],
    last_move: Option<(Piece, Square, Square)>,
) -> bool {
    pieces
        .iter()
        .filter(|piece| piece.colour == player_colour)
        .flat_map(|piece| piece.legal_moves(pieces_vec, last_move))
        .count()
        > 0
}
