use bevy::prelude::*;

pub use creation::create_pieces;

use crate::board::{Promote, Taken};
use crate::pieces::resources::{Meshes, PieceMesh};
use crate::pieces::{Piece, PieceColour};

mod creation;

pub fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece), Without<Taken>>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction =
            Vec3::new(piece.pos.rank as f32, 0.0, piece.pos.file as f32) - transform.translation;

        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

pub fn change_mesh(
    mut commands: Commands,
    meshes: Res<Meshes>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut promoted: Query<(Entity, &mut Piece, &Promote)>,
    children: Query<(&Parent, Entity)>,
) {
    for (entity, mut piece, promotion) in promoted.iter_mut() {
        dbg!(&piece);

        let mesh = meshes
            .0
            .iter()
            .find(|mesh| mesh.matches_type(promotion.to))
            .unwrap()
            .clone();

        piece.piece_type = promotion.to;

        for (parent, child) in children.iter() {
            if parent.get() == entity {
                commands.entity(entity).remove_children(&[child]);
                commands.entity(child).despawn();
            }
        }

        add_new_mesh(&mut commands, &mut materials, entity, &mut piece, mesh);

        commands.entity(entity).remove::<Promote>();
    }
}

fn add_new_mesh(
    mut commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    entity: Entity,
    piece: &mut Mut<Piece>,
    mesh: PieceMesh,
) {
    let material = match piece.colour {
        PieceColour::White => materials.add(Color::rgb(1.0, 0.8, 0.8).into()),
        PieceColour::Black => materials.add(Color::rgb(0.0, 0.2, 0.2).into()),
    };

    use PieceMesh::*;
    match mesh {
        King(mesh_1, mesh_2, transform) | Knight(mesh_1, mesh_2, transform) => {
            let (child_1, child_2) = (
                spawn_child(&mut commands, mesh_1, material.clone(), transform),
                spawn_child(&mut commands, mesh_2, material.clone(), transform),
            );
            commands.entity(entity).add_child(child_1);
            commands.entity(entity).add_child(child_2);
        }
        Queen(mesh, transform)
        | Rook(mesh, transform)
        | Pawn(mesh, transform)
        | Bishop(mesh, transform) => {
            let child = spawn_child(&mut commands, mesh, material.clone(), transform);
            commands.entity(entity).add_child(child);
        }
    }
}

fn spawn_child(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
) -> Entity {
    let child = commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        })
        .id();
    child
}
