use crate::board::PlayerTurn;
use crate::pieces::PieceColour;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .add_startup_system(init_next_move_text)
            .add_system(next_move_text_update);
    }
}

/// Marker component for the Text entity
#[derive(Component)]
struct NextMoveText;

/// Updates the current move text based on the `PlayerTurn` resource
fn next_move_text_update(turn: Res<PlayerTurn>, mut query: Query<(&mut Text, &NextMoveText)>) {
    if !turn.is_changed() {
        return;
    }

    for (mut text, _) in query.iter_mut() {
        text.sections[0].value = format!(
            "Next move: {}",
            match turn.0 {
                PieceColour::White => "White",
                PieceColour::Black => "Black",
            }
        );
    }
}

/// Initialises UiCamera and text
fn init_next_move_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Next move: White",
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    ),
                    ..Default::default()
                },
                NextMoveText,
            ));
        });
}
