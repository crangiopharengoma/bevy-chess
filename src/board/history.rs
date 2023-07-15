use bevy::prelude::{DetectChanges, Local, Query, Res, ResMut, Resource, Without};

use crate::board;
use crate::board::creation::Square;
use crate::board::movement::Taken;
use crate::board::movement::{MoveMadeEvent, MoveStack, MoveType};
use crate::board::status::GameStatus;
use crate::pieces::{Piece, PieceColour, PieceType};

#[derive(Resource, Default)]
pub struct MoveHistory(pub Vec<String>);

pub fn update_move_history(
    mut move_number: Local<u32>,
    move_stack: Res<MoveStack>,
    mut move_history: ResMut<MoveHistory>,
    game_status: Res<GameStatus>,
    pieces: Query<&Piece, Without<Taken>>,
) {
    if move_stack.stack.is_empty() || !game_status.is_changed() {
        return;
    }

    let pieces_vec: Vec<_> = pieces.iter().copied().collect();
    let (last_move, _) = move_stack.stack.last().unwrap();
    let (moving_piece, destination) = (last_move.piece, last_move.destination);

    if moving_piece.colour == PieceColour::White {
        *move_number += 1;
        let move_annotation = generate_move_annotation(
            &format!("{}. ", *move_number),
            last_move,
            &moving_piece,
            &pieces_vec,
            &destination,
            game_status.as_ref(),
        );
        move_history.0.push(move_annotation);
    } else {
        let current = move_history.0.last_mut().unwrap();
        *current = generate_move_annotation(
            current,
            last_move,
            &moving_piece,
            &pieces_vec,
            &destination,
            game_status.as_ref(),
        );
    }
}

fn generate_move_annotation(
    prefix: &str,
    last_move: &MoveMadeEvent,
    moving_piece: &Piece,
    pieces: &[Piece],
    destination: &Square,
    status: &GameStatus,
) -> String {
    let disambiguation = disambiguate_piece(last_move, moving_piece, pieces, destination);

    let status = match status {
        GameStatus::Check => "!",
        GameStatus::Checkmate => "#",
        _ => "",
    };

    match last_move.move_type {
        MoveType::Take(_) | MoveType::TakeEnPassant(_) => {
            let piece_letter = if moving_piece.piece_type == PieceType::Pawn {
                moving_piece.pos.file_annotation()
            } else {
                moving_piece.piece_type.notation_letter()
            };
            format!("{prefix} {piece_letter}{disambiguation}x{destination}{status}")
        }
        MoveType::Castle => {
            if destination.file == board::G_FILE {
                format!("{prefix} 0-0{status}")
            } else {
                format!("{prefix} 0-0-0{status}")
            }
        }
        MoveType::Move => {
            format!(
                "{prefix} {}{disambiguation}{destination}{status}",
                moving_piece.piece_type.notation_letter()
            )
        }
    }
}

fn disambiguate_piece(
    last_move: &MoveMadeEvent,
    moving_piece: &Piece,
    pieces: &[Piece],
    destination: &Square,
) -> String {
    let ambiguous_pieces: Vec<_> = pieces
        .iter()
        .filter(|piece| {
            piece.colour == moving_piece.colour
                && piece.piece_type == moving_piece.piece_type
                && piece
                    .legal_moves(pieces, Some(last_move))
                    .contains(destination)
                && piece.pos != moving_piece.pos
        })
        .collect();

    let file_ambiguous = ambiguous_pieces
        .iter()
        .any(|piece| piece.pos.file == moving_piece.pos.file);
    let rank_ambiguous = ambiguous_pieces
        .iter()
        .any(|piece| piece.pos.rank == moving_piece.pos.rank);

    if file_ambiguous {
        if rank_ambiguous {
            moving_piece.pos.to_string()
        } else {
            moving_piece.pos.rank_annotation()
        }
    } else if !ambiguous_pieces.is_empty() {
        moving_piece.pos.file_annotation()
    } else {
        String::new()
    }
}
