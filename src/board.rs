use crate::pieces::{Piece, PieceColour, PieceType};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_mod_picking::{Highlighting, PickableBundle, PickingEvent, Selection, SelectionEvent};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .add_event::<ResetSelectedEvent>()
            .add_startup_system(create_board)
            .add_system(select_square)
            .add_system(select_piece)
            .add_system(move_piece.before(select_piece)) // if select piece happens first move piece can deselect the selected piece, causing nothing to happen
            .add_system(despawn_taken_pieces)
            .add_system(reset_selected);
    }
}

struct ResetSelectedEvent;

#[derive(Component)]
struct Taken;

#[derive(Resource)]
pub struct PlayerTurn(pub PieceColour);

impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColour::White)
    }
}

impl PlayerTurn {
    fn change(&mut self) {
        self.0 = self.0.opponent()
    }
}

#[derive(Clone, Copy, Component, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Square {
    pub x: i8,
    pub y: i8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }

    /// Returns true if `other` is adjacent to `self`. Adjacency includes diagonals
    ///
    /// Note: returns false if other == self
    pub fn is_adjacent(&self, other: &Square) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    /// Returns true if `other` is in the same rank as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_rank(&self, other: &Square) -> bool {
        self.y == other.y
    }

    /// Returns true if `other` is in the same file as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_file(&self, other: &Square) -> bool {
        self.x == other.x
    }

    /// Returns true if `other` is on the same diagonal as `self`
    ///
    /// Note: returns true if other == self
    pub fn is_same_diagonal(&self, other: &Square) -> bool {
        (self.x - other.x).abs() == (self.y - other.y).abs()
    }

    /// Checks if a piece in the supplied slice of `Piece` occupies the current square
    ///
    /// Returns `None` if `self` is empty, otherwise returns `Some(PieceColour)` of the
    /// piece occupying `self`
    pub fn is_occupied(&self, pieces: &[Piece]) -> Option<PieceColour> {
        pieces
            .iter()
            .find(|piece| *self == piece.pos)
            .map(|piece| piece.colour)
    }
}

impl From<(i8, i8)> for Square {
    fn from((x, y): (i8, i8)) -> Self {
        Square { x, y }
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

fn select_square(
    mut events: EventReader<PickingEvent>,
    mut selected_square: ResMut<SelectedSquare>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(event) = event {
            match event {
                SelectionEvent::JustSelected(entity) => {
                    // println!("New square selected {entity:?}");
                    selected_square.entity = Some(*entity);
                }
                SelectionEvent::JustDeselected(entity) => {
                    // JustDeselected fires when the user is unselecting the current square or when
                    // they select a new square (the previously selected square is unselected. So we
                    // should only clear the SelectedSquare resource when it is the same as the
                    // deselected entity
                    if selected_square.entity == Some(*entity) {
                        selected_square.entity = None;
                    }
                }
            }
        }
    }
}

fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    squares: Query<&Square>,
    pieces: Query<(Entity, &Piece)>,
) {
    if !selected_square.is_changed() {
        return;
    }

    let square = if let Some(Ok(square)) = selected_square
        .entity
        .map(|square_entity| squares.get(square_entity))
    {
        square
    } else {
        return;
    };

    if selected_piece.entity.is_none() {
        selected_piece.entity = pieces
            .iter()
            .find(|(_, piece)| piece.pos == *square && piece.colour == turn.0)
            .map(|(entity, piece)| entity);
    }
}

fn move_piece(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    squares: Query<&Square>,
    mut pieces: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: EventWriter<ResetSelectedEvent>,
) {
    if !selected_square.is_changed() {
        return;
    }

    let square = if let Some(Ok(square)) = selected_square
        .entity
        .map(|square_entity| squares.get(square_entity))
    {
        square
    } else {
        return;
    };

    if let Some(piece_entity) = selected_piece.entity {
        // a piece is selected, so lets move it
        let pieces_vec = pieces.iter_mut().map(|(_, piece)| *piece).collect();

        // this requires a mutable borrow so needs to be done before retrieve the piece that is moving
        let taken_piece = pieces
            .iter_mut()
            .find(|(_, taken_piece)| taken_piece.pos == *square)
            .map(|(entity, _)| entity);

        if let Ok((_, mut piece)) = pieces.get_mut(piece_entity) {
            if piece.is_move_valid(*square, pieces_vec) {
                // take
                if let Some(entity) = taken_piece {
                    commands.entity(entity).insert(Taken);
                }

                // move
                piece.pos = *square;

                // switch turn to opponent
                turn.change();
            }
        }

        reset_selected_event.send(ResetSelectedEvent);
    }
}

fn despawn_taken_pieces(
    mut commands: Commands,
    mut exit_event: EventWriter<AppExit>,
    query: Query<(Entity, &Piece, &Taken)>,
) {
    for (entity, piece, _) in query.iter() {
        // TODO handle mate
        if piece.piece_type == PieceType::King {
            println!(
                "{} won! Thanks for playing!",
                match piece.colour {
                    PieceColour::White => "Black",
                    PieceColour::Black => "White",
                }
            );
            exit_event.send(AppExit);
        }

        commands.entity(entity).despawn_recursive();
    }
}

fn reset_selected(
    mut event_reader: EventReader<ResetSelectedEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut query: Query<&mut Selection>,
) {
    for _ in event_reader.iter() {
        if let Some(square) = selected_square.entity {
            if let Ok(mut selection) = query.get_mut(square) {
                selection.set_selected(false)
            }
        }

        selected_square.entity = None;
        selected_piece.entity = None;
    }
}
