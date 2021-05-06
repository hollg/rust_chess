use bevy::prelude::*;

use super::{Piece, PieceColor, PieceType, is_path_empty};

pub fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (u8, u8),
    asset_server: &AssetServer,
) {
    let mesh: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    commands
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
            piece_type: PieceType::Queen,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

pub fn is_queen_move_valid(
    current_position: (u8, u8),
    target_position: (u8, u8),
    pieces: &Vec<Piece>,
) -> bool {
    let (current_x, current_y) = current_position;
    let (target_x, target_y) = target_position;

    is_path_empty((current_x, current_y), target_position, pieces)
        && ((current_x as i8 - target_x as i8).abs() == (current_y as i8 - target_y as i8).abs()
            || ((current_x == target_x && current_y != target_y)
                || (current_y == target_y && current_x != target_x)))
}
