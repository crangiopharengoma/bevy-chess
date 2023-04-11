use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_mod_picking::{
    Highlighting, Hover, PickableBundle, PickingEvent, Selection, SelectionEvent,
};

use crate::board::components::{Square, Taken};
use crate::board::events::{MoveMadeEvent, ResetSelectedEvent};
use crate::board::resources::{
    Graveyard, PlayerTurn, SelectedPiece, SelectedSquare, SquareMaterials,
};
use crate::pieces::{Piece, PieceType};

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    square_materials: Res<SquareMaterials>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.0,
        subdivisions: 0,
    }));

    for i in 0..8 {
        for j in 0..8 {
            let square = Square { x: i, y: j };
            let initial_material = if square.is_white() {
                square_materials.white_colour.clone()
            } else {
                square_materials.black_colour.clone()
            };
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: initial_material.clone(),
                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                },
                PickableBundle::default(),
                Highlighting {
                    initial: initial_material.clone(),
                    hovered: Some(square_materials.hover_colour.clone()),
                    pressed: None,
                    selected: Some(square_materials.selected_colour.clone()),
                },
                Square { x: i, y: j },
            ));
        }
    }
}

pub fn select_square(
    mut events: EventReader<PickingEvent>,
    mut selected_square: ResMut<SelectedSquare>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(event) = event {
            match event {
                SelectionEvent::JustSelected(entity) => {
                    // println!("New square selected {entity:?}");
                    selected_square.entity = Some(*entity);
                }
                SelectionEvent::JustDeselected(entity) => {
                    // JustDeselected fires when the user is unselecting the current square or when
                    // they select a new square (the previously selected square is unselected. So we
                    // should only clear the SelectedSquare resource when it is the same as the
                    // deselected entity
                    if selected_square.entity == Some(*entity) {
                        selected_square.entity = None;
                    }
                }
            }
        }
    }
}

pub fn colour_moves(
    selected_piece: Res<SelectedPiece>,
    materials: Res<SquareMaterials>,
    pieces: Query<&Piece, Without<Taken>>,
    mut squares: Query<(&Square, &mut Handle<StandardMaterial>, &Selection, &Hover)>,
) {
    let moves = if let Some(piece_entity) = selected_piece.entity {
        let piece = pieces.get(piece_entity).expect("unable to retrieve entity");
        let pieces_vec: Vec<_> = pieces.iter().copied().collect();

        piece.legal_moves(&pieces_vec)
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

pub fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    squares: Query<&Square>,
    pieces: Query<(Entity, &Piece), Without<Taken>>,
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

    if selected_piece.entity.is_none() {
        selected_piece.entity = pieces
            .iter()
            .find(|(_, piece)| piece.pos == *square && piece.colour == turn.0)
            .map(|(entity, _)| entity);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn move_piece(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    mut graveyard: ResMut<Graveyard>,
    squares: Query<&Square>,
    mut pieces: Query<(Entity, &mut Piece), Without<Taken>>,
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
        let pieces_vec: Vec<_> = pieces.iter_mut().map(|(_, piece)| *piece).collect();

        // this requires a mutable borrow so needs to be done before retrieve the piece that is moving
        let taken_piece = pieces
            .iter_mut()
            .find(|(_, taken_piece)| taken_piece.pos == *square)
            .map(|(entity, _)| entity);

        if let Ok((_, mut piece)) = pieces.get_mut(piece_entity) {
            if piece.legal_moves(&pieces_vec).contains(square) {
                // if piece.is_move_valid(*square, pieces_vec) {
                // take
                if let Some(entity) = taken_piece {
                    commands.entity(entity).insert(Taken {
                        grave: graveyard.next(piece.colour),
                    });
                }

                // move
                piece.pos = *square;

                // switch turn to opponent
                turn.change();
                move_made_event.send(MoveMadeEvent {
                    piece: *piece,
                    square: *square,
                });
            }
        }

        reset_selected_event.send(ResetSelectedEvent);
    }
}

pub fn remove_taken_pieces(
    mut exit_event: EventWriter<AppExit>,
    time: Res<Time>,
    mut query: Query<(&Piece, &Taken, &mut Transform)>,
) {
    for (piece, taken, mut transform) in query.iter_mut() {
        // TODO handle mate
        if piece.piece_type == PieceType::King {
            println!("{} won! Thanks for playing!", piece.colour);
            exit_event.send(AppExit);
        }

        let direction = taken.grave - transform.translation;
        // TODO handle piece square not being reset

        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds() * 5.0;
        }
    }
}

pub fn reset_selected(
    mut event_reader: EventReader<ResetSelectedEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut query: Query<&mut Selection>,
) {
    for _ in event_reader.iter() {
        if let Some(square) = selected_square.entity {
            if let Ok(mut selection) = query.get_mut(square) {
                selection.set_selected(false)
            }
        }

        selected_square.entity = None;
        selected_piece.entity = None;
    }
}
