use std::fmt::{Display, Formatter};
use std::ops::Add;

use bevy::math::Vec3;
use bevy::prelude::*;

use crate::board;
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

/// Marker component to indicate when a piece or square is selected
#[derive(Component)]
pub struct Selected;

#[derive(Clone, Copy, Component, PartialEq, Eq, Hash)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Square {
    pub rank: i8,
    pub file: i8,
}

/// Display the square using algebraic notation
impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file_annotation(), self.rank_annotation())
    }
}

impl Square {
    pub fn is_white(&self) -> bool {
        (self.rank + self.file + 1) % 2 == 0
    }

    pub fn file_annotation(&self) -> String {
        match self.file {
            board::A_FILE => "a",
            board::B_FILE => "b",
            board::C_FILE => "c",
            board::D_FILE => "d",
            board::E_FILE => "e",
            board::F_FILE => "f",
            board::G_FILE => "g",
            board::H_FILE => "h",
            _ => panic!("impossible file"),
        }
        .to_string()
    }

    pub fn rank_annotation(&self) -> String {
        (self.rank + 1).to_string()
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
        self.rank >= board::RANK_1
            && self.rank <= board::RANK_8
            && self.file >= board::A_FILE
            && self.file <= board::H_FILE
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
