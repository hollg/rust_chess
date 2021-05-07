use bevy::prelude::*;

use super::{color_of_square, is_path_empty, Piece, PieceColor, PieceType};

pub fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (u8, u8),
    asset_server: &AssetServer,
) {
    let mesh: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");

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
            piece_type: PieceType::Pawn,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

pub fn is_white_pawn_move_valid(
    current_position: (u8, u8),
    target_position: (u8, u8),
    pieces: &Query<&Piece>,
) -> bool {
    let (current_x, current_y) = current_position;
    let (target_x, target_y) = target_position;

    // Normal move
    if target_x as i8 - current_x as i8 == 1 && (current_y == target_y) {
        if color_of_square(target_position, pieces).is_none() {
            return true;
        }
    }

    // Move 2 squares
    if current_x == 1
        && target_x as i8 - current_x as i8 == 2
        && (current_y == target_y)
        && is_path_empty((current_x, current_y), target_position, pieces)
    {
        if color_of_square(target_position, pieces).is_none() {
            return true;
        }
    }

    // Take piece
    if target_x as i8 - current_x as i8 == 1 && (current_y as i8 - target_y as i8).abs() == 1 {
        if color_of_square(target_position, pieces) == Some(PieceColor::Black) {
            return true;
        }
    }

    false
}

pub fn is_black_pawn_move_valid(
    current_position: (u8, u8),
    target_position: (u8, u8),
    pieces: &Query<&Piece>,
) -> bool {
    let (current_x, current_y) = current_position;
    let (target_x, target_y) = target_position;

    // Normal move
    if target_x as i8 - current_x as i8 == -1 && (current_y == target_y) {
        if color_of_square(target_position, pieces).is_none() {
            return true;
        }
    }

    // Move 2 squares
    if current_x == 6
        && target_x as i8 - current_x as i8 == -2
        && (current_y == target_y)
        && is_path_empty((current_x, current_y), target_position, pieces)
    {
        if color_of_square(target_position, pieces).is_none() {
            return true;
        }
    }

    // Take piece
    if target_x as i8 - current_x as i8 == -1 && (current_y as i8 - target_y as i8).abs() == 1 {
        if color_of_square(target_position, pieces) == Some(PieceColor::White) {
            return true;
        }
    }
    false
}
