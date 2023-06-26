use bevy::prelude::*;

use crate::board::{DrawReason, GameStatus, PlayerTurn, PromotionOutcome, SelectPromotionOutcome};
use crate::pieces::PieceType;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .add_startup_system(init_next_move_text)
            .add_system(make_promotion_choice)
            .add_system(display_promotion_menu)
            .add_system(next_move_text_update);
    }
}

/// Marker component for the Text entity
#[derive(Component)]
struct NextMoveText;

/// Marker component for the promotion menu
#[derive(Component)]
struct PromotionMenu {
    promoting_entity: Entity,
}

#[derive(Component)]
struct ButtonValue {
    piece_type: PieceType,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

fn make_promotion_choice(
    mut commands: Commands,
    mut event_writer: EventWriter<PromotionOutcome>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonValue),
        (Changed<Interaction>, With<Button>),
    >,
    menu_query: Query<(Entity, &PromotionMenu)>,
) {
    for (interaction, mut color, button_value) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                for (entity, promotion_menu) in menu_query.iter() {
                    let promotion = PromotionOutcome {
                        entity: promotion_menu.promoting_entity,
                        piece_type: button_value.piece_type,
                    };

                    event_writer.send(promotion);
                    commands.entity(entity).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn display_promotion_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<SelectPromotionOutcome>,
) {
    for event in event_reader.iter() {
        let promoting_entity = event.entity;
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                PromotionMenu { promoting_entity },
            ))
            .with_children(|parent| {
                spawn_button(&asset_server, parent, PieceType::Queen);
                spawn_button(&asset_server, parent, PieceType::Rook);
                spawn_button(&asset_server, parent, PieceType::Bishop);
                spawn_button(&asset_server, parent, PieceType::Knight);
            });
    }
}

fn spawn_button(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder, piece_type: PieceType) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            ButtonValue { piece_type },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                piece_type.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
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
