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
    pub fn is_move_valid(&self, new_position: Square, pieces: Vec<Piece>) -> bool {
        systems::is_move_valid(self, new_position, pieces)
    }
}
