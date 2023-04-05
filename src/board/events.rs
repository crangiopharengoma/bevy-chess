use crate::board::components::Square;
use crate::pieces::Piece;

pub struct MoveMadeEvent {
    pub piece: Piece,
    pub square: Square,
}

pub struct ResetSelectedEvent;
