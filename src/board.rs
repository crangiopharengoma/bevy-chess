use bevy::prelude::*;
// use bevy_mod_picking::PickingEvent::Selection;
use bevy_mod_picking::{Highlighting, PickableBundle, PickingEvent, SelectionEvent};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .add_startup_system(create_board)
            .add_system(colour_squares)
            .add_system(select_square);
    }
}

#[derive(Component)]
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
// fn select_square(query: Query<(&Selection, Entity)>, mut selected_square: ResMut<SelectedSquare>) {
//     if let Some((_, entity)) = query.iter().find(|(selection, _)| selection.selected()) {
//         selected_square.entity = Some(entity);
//     } else {
//         selected_square.entity = None;
//     }
// }

// Event based version
fn select_square(
    mut events: EventReader<PickingEvent>,
    mut selected_square: ResMut<SelectedSquare>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(event) = event {
            selected_square.entity = match event {
                SelectionEvent::JustSelected(entity) => Some(*entity),
                SelectionEvent::JustDeselected(_) => None,
            }
        }
    }
}
