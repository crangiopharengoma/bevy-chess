use bevy::math::Vec3;
use bevy::prelude::Component;

use crate::pieces::{Piece, PieceColour};

#[derive(Component)]
pub struct Taken {
    pub grave: Vec3,
}

#[derive(Clone, Copy, Component, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Square {
    pub x: i8,
    pub y: i8,
}

impl Square {
    pub fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }

    /// Returns true if `other` is adjacent to `self`. Adjacency includes diagonals
    ///
    /// Note: returns false if other == self
    pub fn is_adjacent(&self, other: &Square) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    /// Returns true if `other` is in the same rank as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_rank(&self, other: &Square) -> bool {
        self.y == other.y
    }

    /// Returns true if `other` is in the same file as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_file(&self, other: &Square) -> bool {
        self.x == other.x
    }

    /// Returns true if `other` is on the same diagonal as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_diagonal(&self, other: &Square) -> bool {
        (self.x - other.x).abs() == (self.y - other.y).abs()
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
}

impl From<(i8, i8)> for Square {
    fn from((x, y): (i8, i8)) -> Self {
        Square { x, y }
    }
}
