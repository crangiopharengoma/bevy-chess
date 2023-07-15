use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, Selection, SelectionEvent};

use crate::board::creation::Square;
use crate::board::movement::Taken;
use crate::board::status::PlayerTurn;
use crate::pieces::Piece;

/// Marker component to indicate when a piece or square is selected
#[derive(Component)]
pub struct Selected;

/// Event to signalled that all currently selected pieces and squares should be deselected
pub struct ResetSelectedEvent;

/// Consumes events from Bevy_Mod_Picking and adds the `Selected` marker component when an element
/// is selected, and removes it when it is deselected
pub fn select_square(mut commands: Commands, mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        if let PickingEvent::Selection(event) = event {
            match event {
                SelectionEvent::JustSelected(entity) => {
                    commands.entity(*entity).insert(Selected);
                }
                SelectionEvent::JustDeselected(entity) => {
                    commands.entity(*entity).remove::<Selected>();
                }
            }
        }
    }
}

/// If a selected square contains a piece then give that piece the `Selected` marker trait also
pub fn select_piece(
    mut commands: Commands,
    turn: Res<PlayerTurn>,
    squares: Query<(&Square, &Selected)>,
    pieces: Query<(Entity, &Piece, Option<&Selected>), Without<Taken>>,
) {
    for (square, _) in squares.iter() {
        for (entity, piece, selected) in pieces.iter() {
            if piece.pos.eq(square) && piece.colour == turn.0 && selected.is_none() {
                commands.entity(entity).insert(Selected);
            } else if selected.is_some() && piece.colour == turn.0 && piece.pos.ne(square) {
                commands.entity(entity).remove::<Selected>();
            }
        }
    }
}

pub fn reset_selected(
    mut commands: Commands,
    mut event_reader: EventReader<ResetSelectedEvent>,
    mut selected_squares: Query<(Entity, &Square, &Selected, &mut Selection)>,
    selected_pieces: Query<(Entity, &Piece, &Selected)>,
) {
    for _ in event_reader.iter() {
        for (entity, _, _, mut selection) in selected_squares.iter_mut() {
            selection.set_selected(false);
            commands.entity(entity).remove::<Selected>();
        }

        for (entity, _, _) in selected_pieces.iter() {
            commands.entity(entity).remove::<Selected>();
        }
    }
}
