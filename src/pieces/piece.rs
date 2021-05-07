use bevy::prelude::*;

use super::{
    is_bishop_move_valid, is_black_pawn_move_valid, is_king_move_valid, is_knight_move_valid,
    is_queen_move_valid, is_rook_move_valid, is_white_pawn_move_valid, spawn_bishop, spawn_king,
    spawn_knight, spawn_pawn, spawn_queen, spawn_rook,
};
pub struct Taken;
#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // Current position
    pub x: u8,
    pub y: u8,
}

impl Piece {
    /// Returns the possible_positions that are available
    pub fn can_reach_position(&self, target_position: (u8, u8), pieces: &Query<&Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(target_position, &pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => is_king_move_valid((self.x, self.y), target_position),
            PieceType::Queen => is_queen_move_valid((self.x, self.y), target_position, pieces),
            PieceType::Bishop => is_bishop_move_valid((self.x, self.y), target_position, pieces),
            PieceType::Knight => is_knight_move_valid((self.x, self.y), target_position),
            PieceType::Rook => is_rook_move_valid((self.x, self.y), target_position, pieces),
            PieceType::Pawn => match self.color {
                PieceColor::Black => {
                    is_black_pawn_move_valid((self.x, self.y), target_position, pieces)
                }
                PieceColor::White => {
                    is_white_pawn_move_valid((self.x, self.y), target_position, pieces)
                }
            },
        }
    }

    pub fn is_move_valid(
        &self,
        target_position: (u8, u8),
        pieces: &Query<&Piece>,
        is_check: bool,
    ) -> bool {
        // can the piece reach the position?
        if !self.can_reach_position(target_position, pieces) {
            return false;
        }

        // if there is check, does this move resolve it?
        if is_check {
            // get new set of pieces
            let pieces_copy: Vec<Piece> = pieces.iter().cloned().collect();
            let mut filtered: Vec<Piece> = pieces_copy
                .into_iter()
                .filter(|piece| !(piece.x == self.x && piece.y == self.y))
                .collect();

            filtered.push(Piece {
                x: target_position.0,
                y: target_position.1,
                ..*self
            });

            // check for check
            // if can_any_piece_reach_king(king, &filtered) {
            //     return false;
            // }
        }

        // does this move create check against own king?

        true
    }
}
pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_pieces.system())
            .add_system(move_pieces.system());
    }
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add some materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    spawn_rook(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        (0, 0),
        &asset_server,
    );
    spawn_knight(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        (0, 1),
        &asset_server,
    );
    spawn_bishop(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        (0, 2),
        &asset_server,
    );
    spawn_queen(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        (0, 3),
        &asset_server,
    );
    spawn_king(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        (0, 4),
        &asset_server,
    );
    spawn_bishop(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        (0, 5),
        &asset_server,
    );
    spawn_knight(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        (0, 6),
        &asset_server,
    );
    spawn_rook(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        (0, 7),
        &asset_server,
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            (1, i),
            &asset_server,
        );
    }

    spawn_rook(
        &mut commands,
        black_material.clone(),
        PieceColor::White,
        (7, 0),
        &asset_server,
    );
    spawn_knight(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        (7, 1),
        &asset_server,
    );
    spawn_bishop(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        (7, 2),
        &asset_server,
    );
    spawn_queen(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        (7, 3),
        &asset_server,
    );
    spawn_king(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        (7, 4),
        &asset_server,
    );
    spawn_bishop(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        (7, 5),
        &asset_server,
    );
    spawn_knight(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        (7, 6),
        &asset_server,
    );
    spawn_rook(
        &mut commands,
        black_material.clone(),
        PieceColor::White,
        (7, 7),
        &asset_server,
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            (6, i),
            &asset_server,
        );
    }
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

/// Returns None if square is empty, returns a Some with the color if not
pub fn color_of_square(pos: (u8, u8), pieces: &Query<&Piece>) -> Option<PieceColor> {
    for piece in pieces.iter() {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }
    None
}

pub fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Query<&Piece>) -> bool {
    // Same column
    if begin.0 == end.0 {
        for piece in pieces.iter() {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    // Same row
    if begin.1 == end.1 {
        for piece in pieces.iter() {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }

    // Diagonals
    let x_diff = (begin.0 as i8 - end.0 as i8).abs();
    let y_diff = (begin.1 as i8 - end.1 as i8).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                // left bottom - right top
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                // left top - right bottom
                (begin.0 + i as u8, begin.1 - i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                // right bottom - left top
                (begin.0 - i as u8, begin.1 + i as u8)
            } else {
                // begin.0 > end.0 && begin.1 > end.1
                // right top - left bottom
                (begin.0 - i as u8, begin.1 - i as u8)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}

pub fn can_any_piece_reach_king(king: &Piece, pieces: &Query<&Piece>) -> bool {
    pieces
        .iter()
        .find(|piece| piece.can_reach_position((king.x, king.y), pieces))
        .is_some()
}
