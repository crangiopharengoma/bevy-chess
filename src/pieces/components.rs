use std::fmt::{Display, Formatter};

use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::board::Square;

/// Type alias to make passing around previous moves more convenient
/// Ordering: Piece that moved, origin, destination
pub type MoveRecord = (Piece, Square, Square);

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
    pub has_moved: bool,
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
    ///
    /// The previous move is required for en passant
    pub fn legal_moves(&self, pieces: &[Piece], last_move: Option<MoveRecord>) -> HashSet<Square> {
        self.get_move_set()
            .into_iter()
            .filter(|destination| {
                self.has_clear_path(destination, pieces)
                    && self.piece_specfic_rules(destination, pieces, &last_move)
                    && self.avoids_check(destination, pieces, &last_move)
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
    fn piece_specfic_rules(
        &self,
        new_position: &Square,
        pieces: &[Piece],
        last_move: &Option<MoveRecord>,
    ) -> bool {
        match self.piece_type {
            // PieceType::King => is_valid_for_king(self, new_position),
            // PieceType::Queen => is_valid_for_queen(self, new_position, pieces),
            // PieceType::Bishop => is_valid_for_bishop(self, new_position, pieces),
            // PieceType::Knight => is_valid_for_knight(self, new_position),
            // PieceType::Rook => is_valid_for_rook(self, new_position, pieces),
            PieceType::Pawn => is_valid_for_pawn(self, new_position, pieces, last_move),
            _ => true,
        }
    }

    /// Tests is moving to a new position will result in check. Returns true if a move is 'safe'
    ///
    /// This method relies on a call to `is_move_valid` so kept separate to avoid recursion nightmares
    fn avoids_check(
        &self,
        new_position: &Square,
        pieces: &[Piece],
        last_move: &Option<MoveRecord>,
    ) -> bool {
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
            .any(|piece| piece.is_move_valid(&own_king.pos, &pieces, last_move))
    }

    /// Checks if it is a valid move for self to move to `Square` given the current position of each
    /// `Piece` in `pieces`
    ///
    /// Will return false if the move is invalid - i.e. the path is blocked or they are unable to
    /// move in the direction required
    ///
    /// Note the subtle distinction between 'valid' and 'legal'. It is a 'valid' move for a pinned
    /// piece to given check to the opposition King, but that piece's set of 'legal' moves would be
    /// empty
    fn is_move_valid(
        &self,
        new_position: &Square,
        pieces: &[Piece],
        last_move: &Option<MoveRecord>,
    ) -> bool {
        if new_position == &self.pos || new_position.is_occupied(pieces) == Some(self.colour) {
            return false;
        }

        match self.piece_type {
            PieceType::King => is_valid_for_king(self, new_position),
            PieceType::Queen => is_valid_for_queen(self, new_position, pieces),
            PieceType::Bishop => is_valid_for_bishop(self, new_position, pieces),
            PieceType::Knight => is_valid_for_knight(self, new_position),
            PieceType::Rook => is_valid_for_rook(self, new_position, pieces),
            PieceType::Pawn => is_valid_for_pawn(self, new_position, pieces, last_move),
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

    /// Determines whether or not a piece can take en passant on its next turn
    ///
    /// Always returns false if piece is not a pawn
    ///
    /// Otherwise, checks that the previous move was a pawn, taking a double step ending along side
    /// this piece
    pub fn may_take_en_passant(
        &self,
        new_position: &Square,
        last_move: &Option<MoveRecord>,
    ) -> bool {
        if self.piece_type != PieceType::Pawn {
            false
        } else if let Some((prev_piece, last_move_origin, last_move_destination)) = last_move {
            prev_piece.piece_type == PieceType::Pawn
                && (last_move_origin.file - last_move_destination.file).abs() == 2
                && (last_move_destination.file - new_position.file).abs() == 1
                && (last_move_destination.rank == new_position.rank)
        } else {
            false
        }
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
    ((piece.pos.file - new_position.file).abs() == 2
        && (piece.pos.rank - new_position.rank).abs() == 1)
        || ((piece.pos.file - new_position.file).abs() == 1
            && (piece.pos.rank - new_position.rank).abs() == 2)
}

fn is_valid_for_pawn(
    piece: &Piece,
    new_position: &Square,
    pieces: &[Piece],
    last_move: &Option<MoveRecord>,
) -> bool {
    let (movement_direction, start_x) = match piece.colour {
        PieceColour::White => (1, 1),
        PieceColour::Black => (-1, 6),
    };

    // Standard
    if new_position.file - piece.pos.file == movement_direction
        && piece.pos.rank == new_position.rank
    {
        return new_position.is_occupied(pieces).is_none();
    }

    // Starting
    if piece.pos.file == start_x
        && new_position.file - piece.pos.file == (2 * movement_direction)
        && piece.pos.rank == new_position.rank
        && is_path_empty(&piece.pos, new_position, pieces)
    {
        return new_position.is_occupied(pieces).is_none();
    }

    // Taking
    let may_take = if new_position.file - piece.pos.file == movement_direction
        && (piece.pos.rank - new_position.rank).abs() == 1
    {
        new_position.is_occupied(pieces) == Some(piece.colour.opponent())
    } else {
        false
    };

    may_take || piece.may_take_en_passant(new_position, last_move)
}

/// Checks if any of the pieces supplied are in the path between the two supplied squares
///
/// This method will accurately search both straight (rank or file) and diagonal paths,
/// but it will not validate that the path is one of those three
fn is_path_empty(begin: &Square, end: &Square, pieces: &[Piece]) -> bool {
    if begin.file == end.file {
        // moving along a rank
        !pieces.iter().any(|piece| {
            piece.pos.file == begin.file
                && ((piece.pos.rank > begin.rank && piece.pos.rank < end.rank)
                    || (piece.pos.rank > end.rank && piece.pos.rank < begin.rank))
        })
    } else if begin.rank == end.rank {
        // moving along a file
        !pieces.iter().any(|piece| {
            piece.pos.rank == begin.rank
                && ((piece.pos.file > begin.file && piece.pos.file < end.file)
                    || (piece.pos.file > end.file && piece.pos.file < begin.file))
        })
    } else {
        // diagonal
        let (x_diff, y_diff) = ((begin.file - end.file).abs(), (begin.rank - end.rank).abs());
        if x_diff == y_diff {
            for i in 1..x_diff {
                let pos: Square = if begin.file < end.file && begin.rank < end.rank {
                    // left bottom - right top
                    (begin.file + i, begin.rank + i).into()
                } else if begin.file < end.file && begin.rank > end.rank {
                    // left top - right bottom
                    (begin.file + i, begin.rank - i).into()
                } else if begin.file > end.file && begin.rank < end.rank {
                    // right bottom - left top
                    (begin.file - i, begin.rank + i).into()
                } else {
                    // right top to left bottom
                    (begin.file - i, begin.rank - i).into()
                };

                if pos.is_occupied(pieces).is_some() {
                    return false;
                }
            }
        }

        true
    }
}
