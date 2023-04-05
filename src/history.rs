use bevy::prelude::*;

use crate::board::{MoveMadeEvent, Square};
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
    square: Square,
}

impl Movement {
    fn new(piece: Piece, square: Square) -> Movement {
        Movement { piece, square }
    }
}

#[derive(Resource, Default)]
struct MoveHistory {
    history: Vec<Movement>,
}

fn record_move(mut move_event: EventReader<MoveMadeEvent>, mut history: ResMut<MoveHistory>) {
    for event in move_event.iter() {
        history
            .history
            .push(Movement::new(event.piece, event.square));
    }
}

fn display_move_history(history: ResMut<MoveHistory>) {
    if !history.is_changed() {
        return;
    }

    history
        .history
        .iter()
        .for_each(|Movement { piece, square }| println!("piece: {piece:?} to {square:?}"));
}
