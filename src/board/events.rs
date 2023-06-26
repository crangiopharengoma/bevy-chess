use bevy::prelude::Entity;

use crate::board::components::Square;
use crate::pieces::PieceType;

pub struct ResetSelectedEvent;

#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MoveMadeEvent {
    pub piece: Entity,
    pub origin: Square,
    pub destination: Square,
    pub move_type: MoveType,
}

#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum MoveType {
    Move,
    Take(Entity),
    TakeEnPassant(Entity),
    Castle,
}

impl MoveMadeEvent {
    pub fn not_castling(
        piece: Entity,
        origin: Square,
        destination: Square,
        taken: Option<Entity>,
        en_passant: bool,
    ) -> MoveMadeEvent {
        let move_type = if let Some(entity) = taken {
            if en_passant {
                MoveType::TakeEnPassant(entity)
            } else {
                MoveType::Take(entity)
            }
        } else {
            MoveType::Move
        };

        MoveMadeEvent {
            piece,
            destination,
            origin,
            move_type,
        }
    }

    pub fn castling(piece: Entity, origin: Square, destination: Square) -> MoveMadeEvent {
        MoveMadeEvent {
            piece,
            destination,
            origin,
            move_type: MoveType::Castle,
        }
    }

    pub fn is_take(&self) -> bool {
        matches!(
            self.move_type,
            MoveType::Take(_) | MoveType::TakeEnPassant(_)
        )
    }
}

pub struct SelectPromotionOutcome {
    pub entity: Entity,
}

pub struct PromotionOutcome {
    pub entity: Entity,
    pub piece_type: PieceType,
}
