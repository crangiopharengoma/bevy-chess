use bevy::prelude::*;

use crate::pieces::PieceType;

pub const PAWN_MESH_TRANSLATION: Vec3 = Vec3::new(-0.2, 0.0, 2.6);
pub const ROOK_MESH_TRANSLATION: Vec3 = Vec3::new(-0.1, 0.0, 1.8);
pub const BISHOP_MESH_TRANSLATION: Vec3 = Vec3::new(-0.1, 0.0, 0.0);
pub const QUEEN_MESH_TRANSLATION: Vec3 = Vec3::new(-0.2, 0.0, -0.95);
pub const KNIGHT_MESH_TRANSLATION: Vec3 = Vec3::new(-0.2, 0.0, 0.9);
pub const KING_MESH_TRANSLATION: Vec3 = Vec3::new(-0.2, 0.0, -1.9);

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PieceMesh {
    Pawn(Handle<Mesh>, Transform),
    Rook(Handle<Mesh>, Transform),
    Bishop(Handle<Mesh>, Transform),
    Queen(Handle<Mesh>, Transform),
    Knight(Handle<Mesh>, Handle<Mesh>, Transform),
    King(Handle<Mesh>, Handle<Mesh>, Transform),
}

impl From<&PieceMesh> for PieceType {
    fn from(value: &PieceMesh) -> Self {
        use PieceMesh as Pm;
        use PieceType as Pt;
        match value {
            Pm::King(_, _, _) => Pt::King,
            Pm::Knight(_, _, _) => Pt::Knight,
            Pm::Queen(_, _) => Pt::Queen,
            Pm::Bishop(_, _) => Pt::Bishop,
            Pm::Rook(_, _) => Pt::Rook,
            Pm::Pawn(_, _) => Pt::Pawn,
        }
    }
}

impl PieceMesh {
    pub fn matches_type(&self, piece_type: PieceType) -> bool {
        PieceType::from(self) == piece_type
    }
}

#[derive(Resource)]
pub struct Meshes(pub [PieceMesh; 6]);

impl FromWorld for Meshes {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        Meshes([
            PieceMesh::King(
                asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
                asset_server.load("models/pieces.glb#Mesh1/Primitive0"),
                mesh_transform(KING_MESH_TRANSLATION),
            ),
            PieceMesh::Queen(
                asset_server.load("models/pieces.glb#Mesh7/Primitive0"),
                mesh_transform(QUEEN_MESH_TRANSLATION),
            ),
            PieceMesh::Rook(
                asset_server.load("models/pieces.glb#Mesh5/Primitive0"),
                mesh_transform(ROOK_MESH_TRANSLATION),
            ),
            PieceMesh::Bishop(
                asset_server.load("models/pieces.glb#Mesh6/Primitive0"),
                mesh_transform(BISHOP_MESH_TRANSLATION),
            ),
            PieceMesh::Knight(
                asset_server.load("models/pieces.glb#Mesh3/Primitive0"),
                asset_server.load("models/pieces.glb#Mesh4/Primitive0"),
                mesh_transform(KNIGHT_MESH_TRANSLATION),
            ),
            PieceMesh::Pawn(
                asset_server.load("models/pieces.glb#Mesh2/Primitive0"),
                mesh_transform(PAWN_MESH_TRANSLATION),
            ),
        ])
    }
}

fn mesh_transform(translation: Vec3) -> Transform {
    let mut transform = Transform::from_translation(translation);
    transform.scale *= Vec3::new(0.2, 0.2, 0.2);
    transform
}
