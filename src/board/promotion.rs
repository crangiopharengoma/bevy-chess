use bevy::prelude::{
    Commands, Component, Entity, EventReader, EventWriter, Query, ResMut, Without,
};

use crate::board;
use crate::board::history::MoveHistory;
use crate::board::movement::{Move, Taken};
use crate::pieces::{Piece, PieceType};

#[derive(Component)]
pub struct Promote {
    pub to: PieceType,
}

/// Outbound event signalling that this entity can be promoted and the player should make a selection
///
/// The board plugin expects that after a `SelectPromotionOutcome` event there will be a subsequent
/// `PromotionOutcomeEvent` that contains the same entity and the player's selection
pub struct SelectPromotionOutcome {
    pub entity: Entity,
}

/// Inbound event signalling that an entity previously signalled as ready for promotion has had a
/// decision made by the play about which piece it should be promoted to
pub struct PromotionOutcome {
    pub entity: Entity,
    pub piece_type: PieceType,
}

pub fn select_promotion(
    mut event_writer: EventWriter<SelectPromotionOutcome>,
    pieces: Query<(Entity, &Piece, &Move), Without<Taken>>,
) {
    for (entity, piece, movement) in pieces.iter() {
        if piece.piece_type == PieceType::Pawn
            && (movement.square.rank == board::RANK_1 || movement.square.rank == board::RANK_8)
        {
            event_writer.send(SelectPromotionOutcome { entity });
        }
    }
}

pub fn promote_piece(
    mut commands: Commands,
    mut move_history: ResMut<MoveHistory>,
    mut event_reader: EventReader<PromotionOutcome>,
) {
    for event in event_reader.iter() {
        let promote = Promote {
            to: event.piece_type,
        };

        commands.entity(event.entity).insert(promote);
        move_history
            .0
            .last_mut()
            .unwrap()
            .push_str(&format!("={}", event.piece_type.notation_letter()));
    }
}
