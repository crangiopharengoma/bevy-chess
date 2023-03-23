use bevy::prelude::*;

#[derive(Clone)]
pub enum Piece {
    Pawn(Handle<Mesh>),
    Rook(Handle<Mesh>),
    Bishop(Handle<Mesh>),
    Queen(Handle<Mesh>),
    Knight(Handle<Mesh>, Handle<Mesh>),
    King(Handle<Mesh>, Handle<Mesh>),
}

pub fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pieces = [
        Piece::King(
            asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
            asset_server.load("models/pieces.glb#Mesh1/Primitive0"),
        ),
        Piece::Pawn(asset_server.load("models/pieces.glb#Mesh2/Primitive0")),
        Piece::Knight(
            asset_server.load("models/pieces.glb#Mesh3/Primitive0"),
            asset_server.load("models/pieces.glb#Mesh4/Primitive0"),
        ),
        Piece::Rook(asset_server.load("models/pieces.glb#Mesh5/Primitive0")),
        Piece::Bishop(asset_server.load("models/pieces.glb#Mesh6/Primitive0")),
        Piece::Queen(asset_server.load("models/pieces.glb#Mesh7/Primitive0")),
    ];

    // let king_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
    // let king_cross_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh1/Primitive0");
    // let pawn_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh2/Primitive0");
    // let knight_1_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh3/Primitive0");
    // let knight_2_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh4/Primitive0");
    // let rook_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh5/Primitive0");
    // let bishop_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh6/Primitive0");
    // let queen_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh7/Primitive0");

    let white_material = materials.add(Color::rgb(1.0, 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0.0, 0.2, 0.2).into());

    spawn_set(&mut commands, white_material, &pieces, (1.0, 0.0));
    spawn_set(&mut commands, black_material, &pieces, (6.0, 7.0));

    // spawn_knight(
    //     &mut commands,
    //     white_material.clone(),
    //     knight_1_handle.clone(),
    //     knight_2_handle.clone(),
    //     Vec3::new(0.0, 0.0, 1.0),
    // );
    //
    // spawn_queen(
    //     &mut commands,
    //     white_material.clone(),
    //     queen_handle.clone(),
    //     Vec3::new(0.0, 0.0, 3.0),
    // );
    //
    // spawn_king(
    //     &mut commands,
    //     white_material.clone(),
    //     king_handle.clone(),
    //     king_cross_handle.clone(),
    //     Vec3::new(0.0, 0.0, 4.0),
    // );
}

fn spawn_set(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    pieces: &[Piece],
    (front_row, back_row): (f32, f32),
) {
    for piece in pieces {
        match &piece {
            Piece::King(_, _) => spawn_king(
                commands,
                material.clone(),
                piece.clone(),
                Vec3::new(back_row, 0.0, 4.0),
            ),
            Piece::Queen(_) => spawn_queen(
                commands,
                material.clone(),
                piece.clone(),
                Vec3::new(back_row, 0.0, 3.0),
            ),
            Piece::Rook(_) => {
                spawn_rook(
                    commands,
                    material.clone(),
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 0.0),
                );
                spawn_rook(
                    commands,
                    material.clone(),
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 7.0),
                );
            }
            Piece::Bishop(_) => {
                spawn_bishop(
                    commands,
                    material.clone(),
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 2.0),
                );
                spawn_bishop(
                    commands,
                    material.clone(),
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 5.0),
                );
            }
            Piece::Knight(_, _) => {
                spawn_knight(
                    commands,
                    material.clone(),
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 1.0),
                );
                spawn_knight(
                    commands,
                    material.clone(),
                    piece.clone(),
                    Vec3::new(back_row, 0.0, 6.0),
                );
            }
            Piece::Pawn(_) => {
                for i in 0..=7 {
                    spawn_pawn(
                        commands,
                        material.clone(),
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
    piece: Piece,
    position: Vec3,
    translation: Vec3,
) {
    let transform = {
        let mut transform = Transform::from_translation(translation);
        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
        transform
    };
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            use Piece::*;
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
    piece: Piece,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece,
        position,
        Vec3::new(-0.2, 0.0, -1.9),
    );
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece,
        position,
        Vec3::new(-0.2, 0.0, 0.9),
    );
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece,
        position,
        Vec3::new(-0.2, 0.0, -0.95),
    )
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece,
        position,
        Vec3::new(-0.1, 0.0, 0.0),
    )
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece,
        position,
        Vec3::new(-0.1, 0.0, 1.8),
    )
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    position: Vec3,
) {
    spawn_piece(
        commands,
        material,
        piece,
        position,
        Vec3::new(-0.2, 0.0, 2.6),
    )
}
