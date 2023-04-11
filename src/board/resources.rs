use bevy::prelude::*;

use crate::pieces::PieceColour;

#[derive(Resource)]
pub struct SquareMaterials {
    pub selected_colour: Handle<StandardMaterial>,
    pub hover_colour: Handle<StandardMaterial>,
    pub black_colour: Handle<StandardMaterial>,
    pub white_colour: Handle<StandardMaterial>,
    pub highlight_colour: Handle<StandardMaterial>,
}

impl FromWorld for SquareMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        SquareMaterials {
            hover_colour: materials.add(Color::rgb(0.1, 0.9, 0.7).into()),
            selected_colour: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            black_colour: materials.add(Color::rgb(0., 0.1, 0.1).into()),
            white_colour: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            highlight_colour: materials.add(Color::rgb(0.3, 0.6, 0.8).into()),
        }
    }
}

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
