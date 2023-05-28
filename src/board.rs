use bevy::prelude::*;

pub use components::Square;
pub use components::Taken;
pub use events::MoveMadeEvent;
pub use events::MoveType;
pub use events::ResetSelectedEvent;
pub use resources::{DrawReason, GameStatus, PlayerTurn};
use resources::{Graveyard, MoveStack, SelectedPiece, SelectedSquare, SquareMaterials};

mod components;
mod events;
mod resources;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .init_resource::<SquareMaterials>()
            .init_resource::<Graveyard>()
            .init_resource::<MoveStack>()
            .init_resource::<GameStatus>()
            .add_event::<ResetSelectedEvent>()
            .add_event::<MoveMadeEvent>()
            .add_startup_system(systems::create_board)
            .add_system(systems::select_square)
            .add_system(systems::select_piece)
            .add_system(systems::move_piece.before(systems::select_piece)) // if select piece happens first move piece can deselect the selected piece, causing nothing to happen
            .add_system(systems::make_move)
            .add_system(systems::remove_taken_pieces)
            .add_system(systems::reset_selected)
            .add_system(systems::colour_moves)
            .add_system(systems::push_move)
            .add_system(systems::update_status.before(systems::make_move));
    }
}
