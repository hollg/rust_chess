use bevy::prelude::*;

use super::{is_path_empty, Piece, PieceColor, PieceType};

pub fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (u8, u8),
    asset_server: &AssetServer,
) {
    let mesh: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");

    commands
        // Spawn parent entity
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Rook,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

pub fn is_rook_move_valid(
    current_position: (u8, u8),
    target_position: (u8, u8),
    pieces: &Query<&Piece>,
) -> bool {
    let (current_x, current_y) = current_position;
    let (target_x, target_y) = target_position;

    is_path_empty((current_x, current_y), target_position, pieces)
        && ((current_x == target_x && current_y != target_y)
            || (current_y == target_y && current_x != target_x))
}
