use bevy::prelude::*;

pub use components::{Piece, PieceColour, PieceType};
use resources::Meshes;

mod components;
mod resources;
mod systems;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .init_resource::<Meshes>()
            .add_startup_system(systems::create_pieces)
            .add_system(systems::change_mesh)
            .add_system(systems::move_pieces);
    }
}
