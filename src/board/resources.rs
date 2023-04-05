use bevy::math::Vec3;
use bevy::prelude::{Entity, Resource};

use crate::pieces::PieceColour;

#[derive(Resource)]
pub struct Graveyard {
    white: Vec3,
    black: Vec3,
}

impl Default for Graveyard {
    fn default() -> Self {
        Graveyard {
            white: Vec3::new(-1.0, 0.0, 0.0),
            black: Vec3::new(8.0, 0.0, 0.0),
        }
    }
}

impl Graveyard {
    pub fn next(&mut self, colour: PieceColour) -> Vec3 {
        match colour {
            PieceColour::White => self.next_white(),
            PieceColour::Black => self.next_black(),
        }
    }

    fn next_white(&mut self) -> Vec3 {
        let current = self.white;
        self.white = if current.z >= 7.0 {
            Vec3::new(current.x - 1.0, current.y, 0.0)
        } else {
            Vec3::new(current.x, current.y, current.z + 1.0)
        };
        current
    }

    fn next_black(&mut self) -> Vec3 {
        let current = self.black;
        self.black = if current.z >= 7.0 {
            Vec3::new(current.x + 1.0, current.y, 0.0)
        } else {
            Vec3::new(current.x, current.y, current.z + 1.0)
        };
        current
    }
}

#[derive(Resource)]
pub struct PlayerTurn(pub PieceColour);

impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColour::White)
    }
}

impl PlayerTurn {
    pub fn change(&mut self) {
        self.0 = self.0.opponent()
    }
}

#[derive(Default, Resource)]
pub struct SelectedSquare {
    pub entity: Option<Entity>,
}

#[derive(Default, Resource)]
pub struct SelectedPiece {
    pub entity: Option<Entity>,
}
