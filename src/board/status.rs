use bevy::prelude::*;

use crate::board::movement::{Move, MoveMadeEvent, MoveStack, Taken};
use crate::pieces::{Piece, PieceColour, PieceType};

#[derive(Resource)]
pub struct PlayerTurn(pub PieceColour);

impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColour::White)
    }
}

impl PlayerTurn {
    pub fn change(&mut self) {
        self.0 = self.0.opponent()
    }
}

#[derive(Resource, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum GameStatus {
    #[default]
    NotStarted,
    OnGoing,
    Check,
    Checkmate,
    Draw(DrawReason),
}

/// The various different rules that can lead to a draw. Fivefold Repetition and DeadPosition are not
/// yet checked. A full implementation of DeadPosition is probably beyond the scope of this project
/// but the intent is to capture simple material based dead positions, but not capture more complex
/// board state scenarios where in theory sufficient material exits for a mate but it is impossible
/// to actually achieve mate.
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum DrawReason {
    Stalemate,
    // FivefoldRepetition,
    FiftyMoveRule,
    // DeadPosition,
}

pub fn update_status(
    mut last_action: Local<i32>,
    move_stack: Res<MoveStack>,
    mut turn: ResMut<PlayerTurn>,
    mut game_status: ResMut<GameStatus>,
    pieces: Query<(&Piece, Option<&Move>), Without<Taken>>,
) {
    if move_stack.stack.is_empty() || !move_stack.is_changed() {
        return;
    }

    let pieces_vec: Vec<_> = pieces
        .iter()
        .map(|(piece, move_opt)| match move_opt {
            None => *piece,
            Some(movement) => {
                let mut piece = *piece;
                piece.pos = movement.square;
                piece.has_moved = true;
                piece
            }
        })
        .collect();

    let (last_move, _) = move_stack.stack.last().unwrap();

    if last_move.piece.piece_type == PieceType::Pawn || last_move.is_take() {
        *last_action = 0
    } else {
        *last_action += 1
    }

    let has_moves = player_has_moves(turn.0.opponent(), &pieces_vec, &pieces_vec, last_move);
    let check = is_in_check(turn.0.opponent(), &pieces_vec, &pieces_vec, last_move);

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
}

fn is_in_check(
    player_colour: PieceColour,
    pieces: &[Piece],
    pieces_vec: &[Piece],
    last_move: &MoveMadeEvent,
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
                .legal_moves(pieces_vec, Some(last_move))
                .contains(&own_king.pos)
        })
}

fn player_has_moves(
    player_colour: PieceColour,
    pieces: &[Piece],
    pieces_vec: &[Piece],
    last_move: &MoveMadeEvent,
) -> bool {
    pieces
        .iter()
        .filter(|piece| piece.colour == player_colour)
        .flat_map(|piece| piece.legal_moves(pieces_vec, Some(last_move)))
        .count()
        > 0
}
