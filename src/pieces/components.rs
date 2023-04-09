use std::fmt::{Display, Formatter};

use bevy::prelude::*;

use crate::board::Square;
use crate::pieces::systems;

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PieceColour {
    White,
    Black,
}

impl PieceColour {
    pub fn opponent(&self) -> PieceColour {
        match self {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
        }
    }
}

impl Display for PieceColour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceColour::White => "White",
                PieceColour::Black => "Black",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy, Component)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Piece {
    pub colour: PieceColour,
    pub piece_type: PieceType,
    pub pos: Square,
}

impl Piece {
    /// Checks if it is a valid move for self to move to `Square` given the current position of each
    /// `Piece` in `pieces`
    ///
    /// Will return false if the move is invalid - i.e. the path is blocked or they are unable to
    /// move in the direction required
    ///
    /// If the move is legal but results in a piece in the target square being (legally) taken will
    /// return true
    pub fn is_move_valid(&self, new_position: Square, pieces: Vec<Piece>) -> bool {
        if new_position == self.pos || new_position.is_occupied(&pieces) == Some(self.colour) {
            return false;
        }

        // TODO must move out of check if possible

        match self.piece_type {
            PieceType::King => is_valid_for_king(self, new_position),
            PieceType::Queen => is_valid_for_queen(self, new_position, &pieces),
            PieceType::Bishop => is_valid_for_bishop(self, new_position, &pieces),
            PieceType::Knight => is_valid_for_knight(self, new_position),
            PieceType::Rook => is_valid_for_rook(self, new_position, &pieces),
            PieceType::Pawn => is_valid_for_pawn(self, new_position, &pieces),
        }
    }
}

fn is_valid_for_king(piece: &Piece, new_position: Square) -> bool {
    piece.pos.is_adjacent(&new_position)
    // TODO castling
    // TODO prevent moving into check
}

fn is_valid_for_queen(piece: &Piece, new_position: Square, pieces: &[Piece]) -> bool {
    is_path_empty(piece.pos, new_position, pieces)
        && (new_position.is_same_diagonal(&piece.pos)
            || new_position.is_same_file(&piece.pos)
            || new_position.is_same_rank(&piece.pos))
}

fn is_valid_for_bishop(piece: &Piece, new_position: Square, pieces: &[Piece]) -> bool {
    is_path_empty(piece.pos, new_position, pieces) && new_position.is_same_diagonal(&piece.pos)
}

fn is_valid_for_rook(piece: &Piece, new_position: Square, pieces: &[Piece]) -> bool {
    is_path_empty(piece.pos, new_position, pieces)
        && (new_position.is_same_file(&piece.pos) || new_position.is_same_rank(&piece.pos))
}

fn is_valid_for_knight(piece: &Piece, new_position: Square) -> bool {
    ((piece.pos.x - new_position.x).abs() == 2 && (piece.pos.y - new_position.y).abs() == 1)
        || ((piece.pos.x - new_position.x).abs() == 1 && (piece.pos.y - new_position.y).abs() == 2)
}

fn is_valid_for_pawn(piece: &Piece, new_position: Square, pieces: &[Piece]) -> bool {
    let (movement_direction, start_x) = match piece.colour {
        PieceColour::White => (1, 1),
        PieceColour::Black => (-1, 6),
    };

    // Standard
    if new_position.x - piece.pos.x == movement_direction && piece.pos.y == new_position.y {
        return new_position.is_occupied(pieces).is_none();
    }

    // Starting
    if piece.pos.x == start_x
        && new_position.x - piece.pos.x == (2 * movement_direction)
        && piece.pos.y == new_position.y
        && is_path_empty(piece.pos, new_position, pieces)
    {
        return new_position.is_occupied(pieces).is_none();
    }

    // Taking
    if new_position.x - piece.pos.x == movement_direction
        && (piece.pos.y - new_position.y).abs() == 1
    {
        return new_position.is_occupied(pieces) == Some(piece.colour.opponent());
    }

    false
    // TODO en passant
}

/// Checks if any of the pieces supplied are in the path between the two supplied squares
///
/// This method will accurately search both straight (rank or file) and diagonal paths,
/// but it will not validate that the path is one of those three
fn is_path_empty(begin: Square, end: Square, pieces: &[Piece]) -> bool {
    if begin.x == end.x {
        // moving along a rank
        !pieces.iter().any(|piece| {
            piece.pos.x == begin.x
                && ((piece.pos.y > begin.y && piece.pos.y < end.y)
                    || (piece.pos.y > end.y && piece.pos.y < begin.y))
        })
    } else if begin.y == end.y {
        // moving along a file
        !pieces.iter().any(|piece| {
            piece.pos.y == begin.y
                && ((piece.pos.x > begin.x && piece.pos.x < end.x)
                    || (piece.pos.x > end.x && piece.pos.x < begin.x))
        })
    } else {
        // diagonal
        let (x_diff, y_diff) = ((begin.x - end.x).abs(), (begin.y - end.y).abs());
        if x_diff == y_diff {
            for i in 1..x_diff {
                let pos: Square = if begin.x < end.x && begin.y < end.y {
                    // left bottom - right top
                    (begin.x + i, begin.y + i).into()
                } else if begin.x < end.x && begin.y > end.y {
                    // left top - right bottom
                    (begin.x + i, begin.y - i).into()
                } else if begin.x > end.x && begin.y < end.y {
                    // right bottom - left top
                    (begin.x - i, begin.y + i).into()
                } else {
                    // right top to left bottom
                    (begin.x - i, begin.y - i).into()
                };

                if pos.is_occupied(pieces).is_some() {
                    return false;
                }
            }
        }

        true
    }
}
