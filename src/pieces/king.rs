use bevy::prelude::*;

use super::{Piece, PieceColor, PieceType};

pub fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (u8, u8),
    asset_server: &Res<AssetServer>,
) {
    let mesh: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let mesh_cross: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
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
            piece_type: PieceType::King,
            x: position.0,
            y: position.1,
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
            parent.spawn_bundle(PbrBundle {
                mesh: mesh_cross,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

pub fn is_king_move_valid(current_position: (u8, u8), target_position: (u8, u8)) -> bool {
    let (current_x, current_y) = current_position;
    let (target_x, target_y) = target_position;

    return ((current_x as i8 - target_x as i8).abs() == 1
                    && (current_y == target_y))
                // Vertical
                || ((current_y as i8 - target_y as i8).abs() == 1
                    && (current_x == target_x))
                // Diagonal
                || ((current_x as i8 - target_x as i8).abs() == 1
                    && (current_y as i8 - target_y as i8).abs() == 1);
}
