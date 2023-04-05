use bevy::prelude::*;

pub use components::{Piece, PieceColour, PieceType};

mod components;
mod systems;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .add_startup_system(systems::create_pieces)
            .add_system(systems::move_pieces);
    }
}
