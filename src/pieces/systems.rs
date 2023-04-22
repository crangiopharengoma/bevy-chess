use bevy::prelude::*;

pub use creation::create_pieces;

use crate::board::Taken;
use crate::pieces::Piece;

mod creation;

pub fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece), Without<Taken>>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction =
            Vec3::new(piece.pos.file as f32, 0.0, piece.pos.rank as f32) - transform.translation;

        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}
