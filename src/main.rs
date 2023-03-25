use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle};

use crate::board::BoardPlugin;
use crate::pieces::create_pieces;

mod board;
mod pieces;

fn main() {
    App::default()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_mod_picking::DebugCursorPickingPlugin)
        .add_plugin(bevy_mod_picking::DebugEventsPickingPlugin)
        .add_plugin(BoardPlugin)
        .add_startup_system(setup)
        .add_startup_system(create_pieces)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_matrix(Mat4::from_rotation_translation(
                    Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                    Vec3::new(-7.0, 20.0, 4.0),
                )),
                ..Default::default()
            },
            PickingCameraBundle::default(),
        ))
        .commands()
        .spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
