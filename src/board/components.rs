use std::ops::Add;

use bevy::math::Vec3;
use bevy::prelude::*;

use crate::pieces::{Piece, PieceColour, PieceType};

#[derive(Component)]
pub struct Taken {
    pub grave: Vec3,
}

#[derive(Component)]
pub struct Move {
    pub square: Square,
}

#[derive(Component)]
pub struct Promote {
    pub to: PieceType,
}

#[derive(Clone, Copy, Component, PartialEq, Eq, Hash)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Square {
    pub rank: i8,
    pub file: i8,
}

impl Square {
    pub fn is_white(&self) -> bool {
        (self.rank + self.file + 1) % 2 == 0
    }

    /// Returns true if `other` is adjacent to `self`. Adjacency includes diagonals
    ///
    /// Note: returns false if other == self
    pub fn is_adjacent(&self, other: &Square) -> bool {
        (self.rank - other.rank).abs() <= 1 && (self.file - other.file).abs() <= 1
    }

    /// Returns true if `other` is in the same rank as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_rank(&self, other: &Square) -> bool {
        self.rank == other.rank
    }

    /// Returns true if `other` is in the same file as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_file(&self, other: &Square) -> bool {
        self.file == other.file
    }

    /// Returns true if `other` is on the same diagonal as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_diagonal(&self, other: &Square) -> bool {
        (self.rank - other.rank).abs() == (self.file - other.file).abs()
    }

    /// Checks if a piece in the supplied slice of `Piece` occupies the current square
    ///
    /// Returns `None` if `self` is empty, otherwise returns `Some(PieceColour)` of the
    /// piece occupying `self`
    pub fn is_occupied(&self, pieces: &[Piece]) -> Option<PieceColour> {
        pieces
            .iter()
            .find(|piece| *self == piece.pos)
            .map(|piece| piece.colour)
    }

    /// Checks if a square is a valid position on a chess board
    ///
    /// True means x and y are both between 0 and 7
    pub fn is_valid(&self) -> bool {
        self.rank >= 0 && self.rank < 8 && self.file >= 0 && self.file < 8
    }

    /// Fallible add operation
    ///
    /// Returns Err(String) if the resulting position would be off the board
    pub fn try_add(&self, rhs: (i8, i8)) -> Result<Square, String> {
        let addition = self + rhs;
        if addition.is_valid() {
            Ok(addition)
        } else {
            Err(String::from("this error message should never be used"))
        }
    }
}

impl Add<(i8, i8)> for Square {
    type Output = Square;

    // this can be delegated to the impl for &Square but clippy thinks that that's a needless cast
    #[allow(clippy::op_ref)]
    fn add(self, rhs: (i8, i8)) -> Self::Output {
        (&self) + rhs
    }
}

impl Add<(i8, i8)> for &Square {
    type Output = Square;

    fn add(self, (rhs_x, rhs_y): (i8, i8)) -> Self::Output {
        Square {
            rank: self.rank + rhs_x,
            file: self.file + rhs_y,
        }
    }
}

impl From<(i8, i8)> for Square {
    fn from((x, y): (i8, i8)) -> Self {
        Square { rank: x, file: y }
    }
}
