use bevy::prelude::*;
// use bevy_mod_picking::PickingEvent::Selection;
use crate::pieces::Piece;
use bevy_mod_picking::{Highlighting, PickableBundle, PickingEvent, Selection, SelectionEvent};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board)
            .add_system(colour_squares)
            .add_system(select_square);
    }
}

#[derive(Clone, Copy, Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default, Resource)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default, Resource)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.0,
        subdivisions: 0,
    }));

    for i in 0..8 {
        for j in 0..8 {
            let square = Square { x: i, y: j };
            let initial_material = if square.is_white() {
                materials.add(Color::rgb(1.0, 0.9, 0.9).into())
            } else {
                materials.add(Color::rgb(0.0, 0.1, 0.1).into())
            };
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: initial_material.clone(),

                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                },
                PickableBundle::default(),
                Highlighting {
                    initial: initial_material.clone(),
                    hovered: Some(materials.add(Color::rgb(0.8, 0.3, 0.3).into())),
                    pressed: None,
                    selected: Some(materials.add(Color::rgb(0.9, 0.1, 0.1).into())),
                },
                Square { x: i, y: j },
            ));
        }
    }
}

fn colour_squares() {}

// Query based version
// fn select_square_query(
//     selection_query: Query<(&Selection, &Square, Entity)>,
//     mut pieces: Query<(&mut Piece, Entity)>,
//     mut selected_square: ResMut<SelectedSquare>,
//     mut selected_piece: ResMut<SelectedPiece>,
// ) {
//     if let Some((_, square, entity)) = selection_query
//         .iter()
//         .find(|(selection, _, _)| selection.selected())
//     {
//         selected_square.entity = Some(entity);
//
//         update_selected_piece(
//             &mut selected_piece,
//             &mut selected_square,
//             &squares,
//             &mut pieces,
//         );
//     } else {
//         selected_square.entity = None;
//     }
// }

// Event based version
fn select_square(
    mut events: EventReader<PickingEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares: Query<&Square>,
    mut pieces: Query<(&mut Piece, Entity)>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(event) = event {
            selected_square.entity = match event {
                SelectionEvent::JustSelected(entity) => {
                    update_selected_piece(&mut selected_piece, *entity, &squares, &mut pieces)
                }
                SelectionEvent::JustDeselected(_) => None,
            }
        }
    }
}

/// Updates the location of the currently selected piece based on the location of the selected square
/// represented by the `selected_square` `Entity`
///
/// If no piece is currently selected, checks if there is a piece at the currently selected location
/// and updates the selected piece
///
/// Returns `None` if a piece is currently selected, otherwise returns `Some(selected_square)`
///
/// # Panics
///
/// Panics if the selected_square entity does not have a `Square` component
///
fn update_selected_piece(
    selected_piece: &mut ResMut<SelectedPiece>,
    selected_square: Entity,
    squares: &Query<&Square>,
    pieces: &mut Query<(&mut Piece, Entity)>,
) -> Option<Entity> {
    let square = squares.get(selected_square).unwrap();

    // a piece is selected, so lets move it
    if let Some(piece_entity) = selected_piece.entity {
        if let Ok((mut piece, _)) = pieces.get_mut(piece_entity) {
            piece.x = square.x;
            piece.y = square.y;
        }

        selected_piece.entity = None;
        None
    } else {
        // no piece currently selected, if one is in the selected square, select it
        selected_piece.entity = pieces
            .iter_mut()
            .find(|(piece, _)| piece.x == square.x && piece.y == square.y)
            .map(|(_, piece_entity)| piece_entity);
        Some(selected_square)
    }
}
