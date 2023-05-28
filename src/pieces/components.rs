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

    /// Returns the standard direction of pawn movement for the colour (i.e. 1 for `White` -1 for
    /// `Black`)
    pub fn pawn_movement_direction(&self) -> i8 {
        match self {
            PieceColour::White => 1,
            PieceColour::Black => -1,
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
    const KING_OFFSETS: [(i8, i8); 10] = [
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
        (0, 2),  // castle
        (0, -2), // queen side castle
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
    /// - Pawns may take en passant
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
            PieceType::King => is_valid_for_king(self, new_position, pieces),
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
    fn avoids_check(&self, new_position: &Square, pieces: &[Piece]) -> bool {
        // updates the position of the moving piece and filters out the taken piece (if any)
        let pieces: Vec<Piece> = pieces
            .iter()
            .filter_map(|piece| {
                if piece.pos == self.pos {
                    let mut piece = *piece;
                    piece.pos = *new_position;
                    Some(piece)
                } else if piece.colour == self.colour || &piece.pos != new_position {
                    Some(*piece)
                } else {
                    None
                }
            })
            .collect();

        let own_king = pieces
            .iter()
            .find(|piece| piece.colour == self.colour && piece.piece_type == PieceType::King)
            .expect("unable to find king");

        !pieces
            .iter()
            .filter(|piece| piece.colour != self.colour)
            .any(|piece| {
                piece.is_move_valid(
                    &own_king.pos,
                    &pieces,
                    &Some((*self, self.pos, *new_position)),
                )
            })
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
            PieceType::King => is_valid_for_king(self, new_position, pieces),
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
                && (last_move_origin.rank - last_move_destination.rank).abs() == 2 // last move was pawn double step
                && (last_move_destination.file - self.pos.file).abs() == 1     // last move was pawn in adjacent file
                && last_move_destination.rank == self.pos.rank // pawn is currently in the same rank
                && new_position.file == last_move_destination.file // move is into the file of the moving pawn (i.e. diagonal)
                && (new_position.rank - last_move_destination.rank).abs() == 1 // move is into the rank behind the moving pawn
        } else {
            false
        }
    }

    /// Tests whether the piece may legally castle to `new_position` with the board state `pieces`
    ///
    /// Will always return false if the piece is not a King
    ///
    /// Otherwise will return true if the King may castle to that location
    ///
    /// Legal castling requires:
    /// - Neither the rook nor the King have moved
    /// - None of the squares on the path that the King moves through are threatened
    ///
    /// Note
    /// - The rook may be threatened at the start of the movement (as the King's path does not
    /// include this space)
    /// - Castling is legal on both King and Queen side of the board. This method will return
    /// true for either side with no further distinction
    pub fn may_castle(&self, new_position: &Square, pieces: &[Piece]) -> bool {
        if !self.has_moved
            && self.piece_type == PieceType::King
            && self.pos.rank == new_position.rank
        // when avoid_check checks this method is called without checking if it's a legal space (because that causes unbounded recursion), so this method can be reached, so we need to double check
        {
            pieces
                .iter()
                .filter(|oth_piece| {
                    oth_piece.piece_type == PieceType::Rook
                        && oth_piece.colour == self.colour
                        && !oth_piece.has_moved
                })
                .any(|rook| {
                    // separate checks for queenside/kingside castling
                    if new_position.file == 2 {
                        rook.pos.file == 0 && is_path_empty(&self.pos, new_position, pieces)
                    } else {
                        rook.pos.file == 7 && is_path_empty(&self.pos, new_position, pieces)
                    }
                })
                && self.no_check_in_path(new_position, pieces)
        } else {
            false
        }
    }

    fn no_check_in_path(&self, new_position: &Square, pieces: &[Piece]) -> bool {
        let path: Vec<Square> = (0..3)
            .map(|step| {
                let direction = if new_position.file > self.pos.file {
                    1
                } else {
                    -1
                };
                Square {
                    rank: self.pos.rank,
                    file: self.pos.file + (step * direction),
                }
            })
            .collect();

        !pieces
            .iter()
            .filter(|opp_piece| {
                opp_piece.colour == self.colour.opponent()
                    && opp_piece.piece_type != PieceType::King // FIXME king excluded to prevent endless recursion which means some illegal positions are now possible
            })
            .any(|opp_piece| {
                path.iter().any(|path_sq| {
                    opp_piece.is_move_valid(
                        path_sq,
                        pieces,
                        &Some((*self, self.pos, *new_position)),
                    )
                })
            })
    }
}

fn is_valid_for_king(piece: &Piece, new_position: &Square, pieces: &[Piece]) -> bool {
    piece.pos.is_adjacent(new_position) || piece.may_castle(new_position, pieces)
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
    ((piece.pos.rank - new_position.rank).abs() == 2
        && (piece.pos.file - new_position.file).abs() == 1)
        || ((piece.pos.rank - new_position.rank).abs() == 1
            && (piece.pos.file - new_position.file).abs() == 2)
}

fn is_valid_for_pawn(
    piece: &Piece,
    new_position: &Square,
    pieces: &[Piece],
    last_move: &Option<MoveRecord>,
) -> bool {
    let movement_direction = piece.colour.pawn_movement_direction();

    // Standard
    if new_position.rank - piece.pos.rank == movement_direction
        && piece.pos.file == new_position.file
    {
        return new_position.is_occupied(pieces).is_none();
    }

    // Starting
    if !piece.has_moved
        && new_position.rank - piece.pos.rank == (2 * movement_direction)
        && piece.pos.file == new_position.file
        && is_path_empty(&piece.pos, new_position, pieces)
    {
        return new_position.is_occupied(pieces).is_none();
    }

    // Taking
    let may_take = if new_position.rank - piece.pos.rank == movement_direction
        && (piece.pos.file - new_position.file).abs() == 1
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
    if begin.rank == end.rank {
        // moving along a rank
        !pieces.iter().any(|piece| {
            piece.pos.rank == begin.rank
                && ((piece.pos.file > begin.file && piece.pos.file < end.file)
                    || (piece.pos.file > end.file && piece.pos.file < begin.file))
        })
    } else if begin.file == end.file {
        // moving along a file
        !pieces.iter().any(|piece| {
            piece.pos.file == begin.file
                && ((piece.pos.rank > begin.rank && piece.pos.rank < end.rank)
                    || (piece.pos.rank > end.rank && piece.pos.rank < begin.rank))
        })
    } else {
        // diagonal
        let (x_diff, y_diff) = ((begin.rank - end.rank).abs(), (begin.file - end.file).abs());
        if x_diff == y_diff {
            for i in 1..x_diff {
                let pos: Square = if begin.rank < end.rank && begin.file < end.file {
                    // left bottom - right top
                    (begin.rank + i, begin.file + i).into()
                } else if begin.rank < end.rank && begin.file > end.file {
                    // left top - right bottom
                    (begin.rank + i, begin.file - i).into()
                } else if begin.rank > end.rank && begin.file < end.file {
                    // right bottom - left top
                    (begin.rank - i, begin.file + i).into()
                } else {
                    // right top to left bottom
                    (begin.rank - i, begin.file - i).into()
                };

                if pos.is_occupied(pieces).is_some() {
                    return false;
                }
            }
        }

        true
    }
}
