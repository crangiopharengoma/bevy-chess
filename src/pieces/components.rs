use std::fmt::{Display, Formatter};

use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::board::Square;

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
    const WHITE_PAWN_OFFSETS: [(i8, i8); 4] = [(1, 0), (2, 0), (1, 1), (1, -1)];
    const BLACK_PAWN_OFFSETS: [(i8, i8); 4] = [(-1, 0), (-2, 0), (-1, -1), (-1, 1)];
    const KNIGHT_OFFSETS: [(i8, i8); 8] = [
        (1, 2),
        (-1, 2),
        (1, -2),
        (-1, -2),
        (2, 1),
        (-2, 1),
        (2, -1),
        (-2, -1),
    ];
    const KING_OFFSETS: [(i8, i8); 8] = [
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
    ];
    const BISHOP_OFFSETS: [(i8, i8); 4] = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    const ROOK_OFFSETS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    /// Returns the set of `Square` that this piece can legally move to given the current state of
    /// pieces in the supplied slice
    ///
    /// This includes checking whether a move may cause check, regardless of initial state. That means
    /// that if that piece's side is already in check, the result will likely be empty unless the
    /// move removes the cause of that check
    pub fn legal_moves(&self, pieces: &[Piece]) -> HashSet<Square> {
        self.get_move_set()
            .into_iter()
            .filter(|destination| {
                self.has_clear_path(destination, pieces)
                    && self.piece_specfic_rules(destination, pieces)
                    && self.avoids_check(destination, pieces)
            })
            .collect()
    }

    /// Validates that there exists a clear path from the pieces current position to its target
    /// destination
    ///
    /// If this piece is a knight this will return true unless the target space is of the same
    /// colour regardless of whether the path is clear
    fn has_clear_path(&self, new_position: &Square, pieces: &[Piece]) -> bool {
        is_path_empty(&self.pos, new_position, pieces)
            && new_position.is_occupied(pieces) != Some(self.colour)
    }

    /// Validates that the movement obeys the various piece specific rules that exist, e.g.
    /// - Pawns may only take on diagonals
    /// - Pawns may only move two steps on their first movement
    /// - Pawns may take en passant (not yet implemented)
    /// - The King may castle (not yet implemented)
    ///
    /// Note that knight movement rules are baked into the is_path_empty method checks rather than
    /// being treated as a piece specific rule
    fn piece_specfic_rules(&self, new_position: &Square, pieces: &[Piece]) -> bool {
        match self.piece_type {
            // PieceType::King => is_valid_for_king(self, new_position),
            // PieceType::Queen => is_valid_for_queen(self, new_position, pieces),
            // PieceType::Bishop => is_valid_for_bishop(self, new_position, pieces),
            // PieceType::Knight => is_valid_for_knight(self, new_position),
            // PieceType::Rook => is_valid_for_rook(self, new_position, pieces),
            PieceType::Pawn => is_valid_for_pawn(self, new_position, pieces),
            _ => true,
        }
    }

    /// Tests is moving to a new position will result in check. Returns true if a move is 'safe'
    ///
    /// This method relies on a call to `is_move_valid` so kept separate to avoid recursion nightmares
    fn avoids_check(&self, new_position: &Square, pieces: &[Piece]) -> bool {
        let mut pieces = pieces.to_vec();

        pieces
            .iter_mut()
            .find(|piece| piece.pos == self.pos)
            .unwrap()
            .pos = *new_position;

        let pieces: Vec<_> = pieces
            .into_iter()
            .filter(|piece| piece.colour == self.colour || &piece.pos != new_position)
            .collect();

        let own_king = pieces
            .iter()
            .find(|piece| piece.colour == self.colour && piece.piece_type == PieceType::King)
            .expect("unable to find king");

        !pieces
            .iter()
            .filter(|piece| piece.colour != self.colour)
            .any(|piece| piece.is_move_valid(&own_king.pos, &pieces))
    }

    /// Checks if it is a valid move for self to move to `Square` given the current position of each
    /// `Piece` in `pieces`
    ///
    /// Will return false if the move is invalid - i.e. the path is blocked or they are unable to
    /// move in the direction required
    ///
    /// Note the subtle distinction between 'valid' and 'legal'. It is a 'valid' move for a pinned
    /// piece to given check to the opposition King, but that piece's set of legal moves would be
    /// empty
    fn is_move_valid(&self, new_position: &Square, pieces: &[Piece]) -> bool {
        if new_position == &self.pos || new_position.is_occupied(pieces) == Some(self.colour) {
            return false;
        }

        match self.piece_type {
            PieceType::King => is_valid_for_king(self, new_position),
            PieceType::Queen => is_valid_for_queen(self, new_position, pieces),
            PieceType::Bishop => is_valid_for_bishop(self, new_position, pieces),
            PieceType::Knight => is_valid_for_knight(self, new_position),
            PieceType::Rook => is_valid_for_rook(self, new_position, pieces),
            PieceType::Pawn => is_valid_for_pawn(self, new_position, pieces),
        }
    }

    /// Calculate the maximum set of possible moves that this piece can make
    ///
    /// This does consider board limits and direction but will not consider check
    /// en passant and blocks
    fn get_move_set(&self) -> HashSet<Square> {
        match self.piece_type {
            PieceType::Pawn => match self.colour {
                PieceColour::White => Piece::WHITE_PAWN_OFFSETS
                    .into_iter()
                    .filter_map(|offset| self.pos.try_add(offset).ok())
                    .collect(),
                PieceColour::Black => Piece::BLACK_PAWN_OFFSETS
                    .into_iter()
                    .filter_map(|offset| self.pos.try_add(offset).ok())
                    .collect(),
            },
            PieceType::Knight => Piece::KNIGHT_OFFSETS
                .into_iter()
                .filter_map(|offset| self.pos.try_add(offset).ok())
                .collect(),
            PieceType::King => Piece::KING_OFFSETS
                .into_iter()
                .filter_map(|offset| self.pos.try_add(offset).ok())
                .collect(),
            PieceType::Bishop => Piece::BISHOP_OFFSETS
                .into_iter()
                .flat_map(|offset| self.multiple_steps(offset))
                .collect(),
            PieceType::Rook => Piece::ROOK_OFFSETS
                .into_iter()
                .flat_map(|offset| self.multiple_steps(offset))
                .collect(),
            PieceType::Queen => Piece::ROOK_OFFSETS
                .into_iter()
                .chain(Piece::BISHOP_OFFSETS.into_iter())
                .flat_map(|offset| self.multiple_steps(offset))
                .collect(),
        }
    }

    /// Helper method for calculating maximal move set for pieces that can move as many spaces as
    /// board state allows
    fn multiple_steps(&self, (offset_x, offset_y): (i8, i8)) -> HashSet<Square> {
        (1..8)
            .filter_map(|step| self.pos.try_add((offset_x * step, offset_y * step)).ok())
            .collect()
    }
}

fn is_valid_for_king(piece: &Piece, new_position: &Square) -> bool {
    piece.pos.is_adjacent(new_position)
    // TODO castling
}

fn is_valid_for_queen(piece: &Piece, new_position: &Square, pieces: &[Piece]) -> bool {
    is_path_empty(&piece.pos, new_position, pieces)
        && (new_position.is_same_diagonal(&piece.pos)
            || new_position.is_same_file(&piece.pos)
            || new_position.is_same_rank(&piece.pos))
}

fn is_valid_for_bishop(piece: &Piece, new_position: &Square, pieces: &[Piece]) -> bool {
    is_path_empty(&piece.pos, new_position, pieces) && new_position.is_same_diagonal(&piece.pos)
}

fn is_valid_for_rook(piece: &Piece, new_position: &Square, pieces: &[Piece]) -> bool {
    is_path_empty(&piece.pos, new_position, pieces)
        && (new_position.is_same_file(&piece.pos) || new_position.is_same_rank(&piece.pos))
}

fn is_valid_for_knight(piece: &Piece, new_position: &Square) -> bool {
    ((piece.pos.x - new_position.x).abs() == 2 && (piece.pos.y - new_position.y).abs() == 1)
        || ((piece.pos.x - new_position.x).abs() == 1 && (piece.pos.y - new_position.y).abs() == 2)
}

fn is_valid_for_pawn(piece: &Piece, new_position: &Square, pieces: &[Piece]) -> bool {
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
        && is_path_empty(&piece.pos, new_position, pieces)
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
fn is_path_empty(begin: &Square, end: &Square, pieces: &[Piece]) -> bool {
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
