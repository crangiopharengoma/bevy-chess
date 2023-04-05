use std::fmt::{Display, Formatter};

use bevy::prelude::*;

use crate::board::{Square, Taken};
use crate::movement;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app // new line
            .add_startup_system(create_pieces)
            .add_system(move_pieces);
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PieceColour {
    White,
    Black,
}

impl PieceColour {
    pub fn opponent(&self) -> PieceColour {
        match self {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
        }
    }
}

impl Display for PieceColour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceColour::White => "White",
                PieceColour::Black => "Black",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl From<&PieceMesh> for PieceType {
    fn from(value: &PieceMesh) -> Self {
        use PieceMesh as Pm;
        use PieceType as Pt;
        match value {
            Pm::King(_, _) => Pt::King,
            Pm::Knight(_, _) => Pt::Knight,
            Pm::Queen(_) => Pt::Queen,
            Pm::Bishop(_) => Pt::Bishop,
            Pm::Rook(_) => Pt::Rook,
            Pm::Pawn(_) => Pt::Pawn,
        }
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PieceMesh {
    Pawn(Handle<Mesh>),
    Rook(Handle<Mesh>),
    Bishop(Handle<Mesh>),
    Queen(Handle<Mesh>),
    Knight(Handle<Mesh>, Handle<Mesh>),
    King(Handle<Mesh>, Handle<Mesh>),
}

#[derive(Clone, Copy, Component)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Piece {
    pub colour: PieceColour,
    pub piece_type: PieceType,
    pub pos: Square,
}

impl Piece {
    pub fn is_move_valid(&self, new_position: Square, pieces: Vec<Piece>) -> bool {
        movement::is_move_valid(self, new_position, pieces)
    }
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece), Without<Taken>>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction =
            Vec3::new(piece.pos.x as f32, 0.0, piece.pos.y as f32) - transform.translation;

        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pieces = [
        PieceMesh::King(
            asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
            asset_server.load("models/pieces.glb#Mesh1/Primitive0"),
        ),
        PieceMesh::Queen(asset_server.load("models/pieces.glb#Mesh7/Primitive0")),
        PieceMesh::Rook(asset_server.load("models/pieces.glb#Mesh5/Primitive0")),
        PieceMesh::Bishop(asset_server.load("models/pieces.glb#Mesh6/Primitive0")),
        PieceMesh::Knight(
            asset_server.load("models/pieces.glb#Mesh3/Primitive0"),
            asset_server.load("models/pieces.glb#Mesh4/Primitive0"),
        ),
        PieceMesh::Pawn(asset_server.load("models/pieces.glb#Mesh2/Primitive0")),
    ];

    spawn_set(
        &mut commands,
        PieceColour::White,
        &mut materials,
        &pieces,
        (1.0, 0.0),
    );
    spawn_set(
        &mut commands,
        PieceColour::Black,
        &mut materials,
        &pieces,
        (6.0, 7.0),
    );
}

fn spawn_set(
    commands: &mut Commands,
    piece_colour: PieceColour,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pieces: &[PieceMesh],
    (front_row, back_row): (f32, f32),
) {
    let material = match piece_colour {
        PieceColour::White => materials.add(Color::rgb(1.0, 0.8, 0.8).into()),
        PieceColour::Black => materials.add(Color::rgb(0.0, 0.2, 0.2).into()),
    };

    for piece in pieces {
        match &piece {
            PieceMesh::King(_, _) => spawn_king(
                commands,
                material.clone(),
                piece_colour,
                piece.clone(),
                Vec3::new(back_row, 0.0, 4.0),
            ),
            PieceMesh::Queen(_) => spawn_queen(
                commands,
                material.clone(),
                piece_colour,
                piece.clone(),
                Vec3::new(back_row, 0.0, 3.0),
            ),
            PieceMesh::Rook(_) => {
                spawn_rook(
                    commands,
                    material.clone(),
                    piece_colour,
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 0.0),
                );
                spawn_rook(
                    commands,
                    material.clone(),
                    piece_colour,
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 7.0),
                );
            }
            PieceMesh::Bishop(_) => {
                spawn_bishop(
                    commands,
                    material.clone(),
                    piece_colour,
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 2.0),
                );
                spawn_bishop(
                    commands,
                    material.clone(),
                    piece_colour,
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 5.0),
                );
            }
            PieceMesh::Knight(_, _) => {
                spawn_knight(
                    commands,
                    material.clone(),
                    piece_colour,
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 1.0),
                );
                spawn_knight(
                    commands,
                    material.clone(),
                    piece_colour,
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 6.0),
                );
            }
            PieceMesh::Pawn(_) => {
                for i in 0..=7 {
                    spawn_pawn(
                        commands,
                        material.clone(),
                        piece_colour,
                        piece.clone(),
                        Vec3::new(front_row, 0.0, i as f32),
                    );
                }
            }
        }
    }
}

fn spawn_piece(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
    translation: Vec3,
) {
    let transform = {
        let mut transform = Transform::from_translation(translation);
        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
        transform
    };
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            Piece {
                colour: piece_colour,
                piece_type: (&piece).into(), // from impl on ref to allow mesh to be reused later
                pos: Square {
                    x: position.x as i8,
                    y: position.z as i8,
                },
            },
        ))
        .with_children(|parent| {
            use PieceMesh::*;
            match piece {
                King(mesh_1, mesh_2) | Knight(mesh_1, mesh_2) => {
                    spawn_child(mesh_1, material.clone(), parent, transform);
                    spawn_child(mesh_2, material.clone(), parent, transform);
                }
                Queen(mesh) | Rook(mesh) | Pawn(mesh) | Bishop(mesh) => {
                    spawn_child(mesh, material.clone(), parent, transform);
                }
            }
        });
}

fn spawn_child(
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    parent: &mut ChildBuilder,
    transform: Transform,
) {
    parent.spawn(PbrBundle {
        mesh,
        material,
        transform,
        ..Default::default()
    });
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece_colour,
        piece,
        position,
        Vec3::new(-0.2, 0.0, -1.9),
    );
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece_colour,
        piece,
        position,
        Vec3::new(-0.2, 0.0, 0.9),
    );
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece_colour,
        piece,
        position,
        Vec3::new(-0.2, 0.0, -0.95),
    )
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece_colour,
        piece,
        position,
        Vec3::new(-0.1, 0.0, 0.0),
    )
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece_colour,
        piece,
        position,
        Vec3::new(-0.1, 0.0, 1.8),
    )
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece_colour,
        piece,
        position,
        Vec3::new(-0.2, 0.0, 2.6),
    )
}
