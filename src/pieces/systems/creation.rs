use bevy::prelude::*;

use crate::board::Square;
use crate::pieces::components::{Piece, PieceColour};
use crate::pieces::resources::{Meshes, PieceMesh};

pub fn create_pieces(
    mut commands: Commands,
    meshes: Res<Meshes>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_set(
        &mut commands,
        PieceColour::White,
        &mut materials,
        &meshes,
        (1.0, 0.0),
    );
    spawn_set(
        &mut commands,
        PieceColour::Black,
        &mut materials,
        &meshes,
        (6.0, 7.0),
    );
}

fn spawn_set(
    commands: &mut Commands,
    piece_colour: PieceColour,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pieces: &Meshes,
    (front_row, back_row): (f32, f32),
) {
    let material = match piece_colour {
        PieceColour::White => materials.add(Color::rgb(1.0, 0.8, 0.8).into()),
        PieceColour::Black => materials.add(Color::rgb(0.0, 0.2, 0.2).into()),
    };

    for piece in pieces.0.iter() {
        match piece {
            PieceMesh::King(_, _, _) => spawn_king(
                commands,
                material.clone(),
                piece_colour,
                piece.clone(),
                Vec3::new(back_row, 0.0, 4.0),
            ),
            PieceMesh::Queen(_, _) => spawn_queen(
                commands,
                material.clone(),
                piece_colour,
                piece.clone(),
                Vec3::new(back_row, 0.0, 3.0),
            ),
            PieceMesh::Rook(_, _) => {
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
            PieceMesh::Bishop(_, _) => {
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
            PieceMesh::Knight(_, _, _) => {
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
            PieceMesh::Pawn(_, _) => {
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
) {
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
                    rank: position.x as i8,
                    file: position.z as i8,
                },
                has_moved: false,
            },
        ))
        .with_children(|parent| {
            use PieceMesh::*;
            match piece {
                King(mesh_1, mesh_2, transform) | Knight(mesh_1, mesh_2, transform) => {
                    spawn_child(mesh_1, material.clone(), parent, transform);
                    spawn_child(mesh_2, material.clone(), parent, transform);
                }
                Queen(mesh, transform)
                | Rook(mesh, transform)
                | Pawn(mesh, transform)
                | Bishop(mesh, transform) => {
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
    spawn_piece(commands, material, piece_colour, piece, position);
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(commands, material, piece_colour, piece, position);
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(commands, material, piece_colour, piece, position)
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(commands, material, piece_colour, piece, position)
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(commands, material, piece_colour, piece, position)
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_colour: PieceColour,
    piece: PieceMesh,
    position: Vec3,
) {
    spawn_piece(commands, material, piece_colour, piece, position)
}
