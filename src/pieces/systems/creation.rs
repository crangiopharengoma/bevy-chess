use bevy::prelude::*;

use crate::board::Square;
use crate::pieces::components::{Piece, PieceColour, PieceType};

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

impl From<&PieceMesh> for PieceType {
    fn from(value: &PieceMesh) -> Self {
        use crate::pieces::systems::creation::PieceMesh as Pm;
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

pub fn create_pieces(
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
