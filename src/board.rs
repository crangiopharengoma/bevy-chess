use bevy::prelude::*;

pub use creation::{Square, SquareMaterials};
pub use history::MoveHistory;
pub use movement::{Graveyard, MoveMadeEvent, MoveStack, MoveType, Taken};
pub use promotion::{Promote, PromotionOutcome, SelectPromotionOutcome};
pub use selection::ResetSelectedEvent;
pub use status::{DrawReason, GameStatus, PlayerTurn};

mod creation;
mod history;
mod movement;
mod promotion;
mod selection;
mod status;

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
            // .init_resource::<SelectedSquare>()
            // .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .init_resource::<SquareMaterials>()
            .init_resource::<Graveyard>()
            .init_resource::<MoveStack>()
            .init_resource::<MoveHistory>()
            .init_resource::<GameStatus>()
            .add_event::<ResetSelectedEvent>()
            .add_event::<MoveMadeEvent>()
            .add_event::<SelectPromotionOutcome>()
            .add_event::<PromotionOutcome>()
            .add_startup_system(creation::create_board)
            .add_system(selection::select_square)
            .add_system(selection::select_piece)
            .add_system(selection::reset_selected)
            .add_system(movement::move_piece)
            .add_system(movement::make_move)
            .add_system(movement::remove_taken_pieces)
            .add_system(movement::colour_moves)
            .add_system(movement::push_move)
            .add_system(promotion::select_promotion)
            .add_system(promotion::promote_piece)
            .add_system(history::update_move_history)
            .add_system(status::update_status);
    }
}
