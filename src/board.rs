use bevy::prelude::*;

pub use components::{Promote, Square, Taken};
pub use events::{
    MoveMadeEvent, MoveType, PromotionOutcome, ResetSelectedEvent, SelectPromotionOutcome,
};
pub use resources::{DrawReason, GameStatus, PlayerTurn};
use resources::{Graveyard, MoveStack, SelectedPiece, SelectedSquare, SquareMaterials};

mod components;
mod events;
mod resources;
mod systems;

pub struct BoardPlugin;

pub const A_FILE: i8 = 0;
pub const B_FILE: i8 = 1;
pub const C_FILE: i8 = 2;
pub const D_FILE: i8 = 3;
pub const E_FILE: i8 = 4;
pub const F_FILE: i8 = 5;
pub const G_FILE: i8 = 6;
pub const H_FILE: i8 = 7;

pub const RANK_1: i8 = 0;
pub const RANK_2: i8 = 1;
pub const RANK_3: i8 = 2;
pub const RANK_4: i8 = 3;
pub const RANK_5: i8 = 4;
pub const RANK_6: i8 = 5;
pub const RANK_7: i8 = 6;
pub const RANK_8: i8 = 7;

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
            .add_event::<SelectPromotionOutcome>()
            .add_event::<PromotionOutcome>()
            .add_startup_system(systems::create_board)
            .add_system(systems::select_square)
            .add_system(systems::select_piece)
            .add_system(systems::move_piece.before(systems::select_piece)) // if select piece happens first move piece can deselect the selected piece, causing nothing to happen
            .add_system(systems::make_move)
            .add_system(systems::remove_taken_pieces)
            .add_system(systems::reset_selected)
            .add_system(systems::colour_moves)
            .add_system(systems::push_move)
            .add_system(systems::select_promotion)
            .add_system(systems::promote_piece)
            .add_system(systems::update_status.before(systems::make_move));
    }
}
