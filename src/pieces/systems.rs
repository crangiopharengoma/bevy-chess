use bevy::prelude::*;

pub use creation::create_pieces;

use crate::board::Taken;
pub use crate::pieces::components::is_move_valid;
use crate::pieces::Piece;

mod creation;

pub fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece), Without<Taken>>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction =
            Vec3::new(piece.pos.x as f32, 0.0, piece.pos.y as f32) - transform.translation;

        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}
