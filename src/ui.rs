use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::board;
use crate::board::{
    DrawReason, GameStatus, MoveMadeEvent, MoveType, PlayerTurn, PromotionOutcome,
    SelectPromotionOutcome, Square,
};
use crate::pieces::{Piece, PieceColour, PieceType};

pub struct UiPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .add_startup_system(init_next_move_text)
            .add_startup_system(display_move_log)
            .add_system(mouse_scroll)
            .add_system(make_promotion_choice)
            .add_system(display_promotion_menu)
            .add_system(next_move_text_update)
            .add_system(update_move_log);
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
struct PromoteButton {
    piece_type: PieceType,
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

#[derive(Component, Default)]
struct MoveNumber(u32);

fn update_move_log(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut move_event: EventReader<MoveMadeEvent>,
    pieces: Query<&Piece>,
    scroll_list: Query<(Entity, &ScrollingList)>,
    mut scroll_list_entries: Query<(&MoveNumber, &mut Text)>,
    mut move_number: Local<u32>,
) {
    for event in move_event.iter() {
        let piece = pieces
            .get(event.piece)
            .expect("unable to find moving piece");

        let destination = event.destination;

        if piece.colour == PieceColour::White {
            *move_number += 1;
            let move_annotation = generate_move_annotation(
                &format!("{}. ", *move_number),
                event,
                piece,
                &destination,
            );

            let (sl_entity, _) = scroll_list.iter().next().unwrap();
            commands.entity(sl_entity).with_children(|parent| {
                create_scroll_list_item(&asset_server, parent, move_annotation, *move_number);
            });
        } else {
            for (move_number_record, mut text) in scroll_list_entries.iter_mut() {
                if move_number_record.0 == *move_number {
                    let current = &text.sections[0].value;
                    let move_annotation =
                        generate_move_annotation(current, event, piece, &destination);
                    text.sections[0].value = move_annotation;
                }
            }
        }
    }
}

fn generate_move_annotation(
    prefix: &str,
    event: &MoveMadeEvent,
    piece: &Piece,
    destination: &Square,
) -> String {
    // TODO check for ambiguous cases
    // TODO Handle check and checkmate

    match event.move_type {
        MoveType::Take(_) | MoveType::TakeEnPassant(_) => {
            let piece_letter = if piece.piece_type == PieceType::Pawn {
                piece.pos.to_string().chars().next().unwrap().to_string()
            } else {
                piece.piece_type.notation_letter()
            };
            format!(
                "{prefix} {piece_letter}x{destination}",
                // piece.piece_type.notation_letter()
            )
        }
        MoveType::Castle => {
            if destination.file == board::G_FILE {
                format!("{prefix} 0-0")
            } else {
                format!("{prefix} 0-0-0")
            }
        }
        MoveType::Move => {
            format!(
                "{prefix} {}{destination}",
                piece.piece_type.notation_letter()
            )
        }
    }
}

fn display_move_log(mut commands: Commands, asset_server: Res<AssetServer>) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                position: UiRect::right(Val::Percent(-75.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::width(Val::Percent(25.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Move History",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            size: Size::height(Val::Px(25.)),
                            ..default()
                        }),
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                size: Size::height(Val::Percent(50.0)),
                                overflow: Overflow::Hidden,
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        max_size: Size::UNDEFINED,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ScrollingList::default(),
                                AccessibilityNode(NodeBuilder::new(Role::List)),
                            ));
                            // .with_children(|parent| {
                            //     // List items
                            //     for i in 0..30 {
                            //         create_scroll_list_item(
                            //             &asset_server,
                            //             parent,
                            //             format!("Item {i}"),
                            //         );
                            //     }
                            // });
                        });
                });
        });
}

fn create_scroll_list_item(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    move_text: String,
    move_number: u32,
) {
    parent.spawn((
        TextBundle::from_section(
            move_text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.,
                color: Color::WHITE,
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        MoveNumber(move_number),
    ));
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}

fn make_promotion_choice(
    mut commands: Commands,
    mut event_writer: EventWriter<PromotionOutcome>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &PromoteButton),
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
            PromoteButton { piece_type },
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
