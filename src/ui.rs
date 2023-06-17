use bevy::prelude::*;

use crate::board::{DrawReason, GameStatus, PlayerTurn, PromotionOutcome, SelectPromotionOutcome};
use crate::pieces::PieceType;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .add_startup_system(init_next_move_text)
            .add_system(make_promotion_choice)
            .add_system(next_move_text_update);
    }
}

/// Marker component for the Text entity
#[derive(Component)]
struct NextMoveText;

fn make_promotion_choice(
    mut event_reader: EventReader<SelectPromotionOutcome>,
    mut event_writer: EventWriter<PromotionOutcome>,
) {
    for event in event_reader.iter() {
        // TODO UI element to prompt the user to select something
        let promotion = PromotionOutcome {
            entity: event.entity,
            piece_type: PieceType::Queen,
        };

        event_writer.send(promotion);
    }
}

/// Updates the current move text based on the `PlayerTurn` resource
fn next_move_text_update(
    turn: Res<PlayerTurn>,
    game_status: Res<GameStatus>,
    mut query: Query<(&mut Text, &NextMoveText)>,
) {
    if !turn.is_changed() && !game_status.is_changed() {
        return;
    }

    let piece_colour = turn.0;
    for (mut text, _) in query.iter_mut() {
        text.sections[0].value = match *game_status {
            GameStatus::NotStarted => "Next move: White".to_string(),
            GameStatus::OnGoing => format!("Next move: {piece_colour}"),
            GameStatus::Check => format!("Check! Next move: {piece_colour}"),
            GameStatus::Checkmate => format!("Checkmate! {piece_colour} wins"),
            GameStatus::Draw(DrawReason::FiftyMoveRule) => {
                "Draw! Fifty consecutive moves without a capture or a pawn movement".to_string()
            }
            GameStatus::Draw(DrawReason::Stalemate) => {
                format!("Draw! Stalemate: {piece_colour} has no legal moves")
            }
        };
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
