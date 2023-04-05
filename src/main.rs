use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle};

use pieces::PiecesPlugin;

use crate::board::BoardPlugin;
use crate::history::HistoryPlugin;
use crate::ui::UiPlugin;

mod board;
mod history;
mod pieces;
mod ui;

fn main() {
    App::default()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(HistoryPlugin)
        .add_startup_system(setup)
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
