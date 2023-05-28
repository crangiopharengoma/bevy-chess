use bevy::prelude::*;

use crate::board::{MoveMadeEvent, MoveType, Square};
use crate::pieces::Piece;

pub struct HistoryPlugin;

impl Plugin for HistoryPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .init_resource::<MoveHistory>()
            .add_system(record_move)
            .add_system(display_move_history);
    }
}

struct Movement {
    piece: Piece,
    origin: Square,
    destination: Square,
    taken: Option<Piece>,
}

impl Movement {
    fn new(piece: Piece, origin: Square, destination: Square, taken: Option<Piece>) -> Movement {
        Movement {
            piece,
            origin,
            destination,
            taken,
        }
    }
}

#[derive(Resource, Default)]
struct MoveHistory {
    history: Vec<Movement>,
}

fn record_move(
    mut move_event: EventReader<MoveMadeEvent>,
    mut history: ResMut<MoveHistory>,
    pieces: Query<&Piece>,
) {
    for event in move_event.iter() {
        let piece = pieces
            .get(event.piece)
            .expect("unable to find moving piece");
        let taken =
            if let MoveType::Take(entity) | MoveType::TakeEnPassant(entity) = event.move_type {
                Some(*pieces.get(entity).expect("unable to find taken piece);"))
            } else {
                None
            };
        history.history.push(Movement::new(
            *piece,
            event.origin,
            event.destination,
            taken,
        ));
    }
}

fn display_move_history(history: ResMut<MoveHistory>) {
    if !history.is_changed() {
        return;
    }

    history.history.iter().for_each(
        |Movement {
             piece,
             destination,
             origin,
             taken
         }| {
            if let Some(taken_piece) = taken {
                println!("piece: {piece:?} move from {origin:?} to {destination:?}, taking {taken_piece:?}");
            } else {
                println!("piece: {piece:?} move from {origin:?} to {destination:?}");
            }
        }
    );
}
