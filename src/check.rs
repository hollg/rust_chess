use bevy::prelude::*;

use crate::{
    board::PlayerTurn,
    pieces::{can_any_piece_reach_king, Piece, PieceType},
};

struct Check {
    pub is_check: bool,
    pub is_checkmate: bool,
}

impl Default for Check {
    fn default() -> Self {
        Self {
            is_check: false,
            is_checkmate: false,
        }
    }
}

pub struct CheckPlugin;
impl Plugin for CheckPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Check>()
            .add_system(check_updater.system());
    }
}

fn check_updater(mut check: ResMut<Check>, turn: Res<PlayerTurn>, pieces_query: Query<&Piece>) {
    if !turn.is_changed() {
        return;
    }

    let pieces: Vec<Piece> = pieces_query
        .iter()
        .collect::<Vec<&Piece>>()
        .into_iter()
        .cloned()
        .collect();

    // find the current player's king
    if let Some(king) = pieces_query
        .iter()
        .find(|piece| piece.color == turn.0 && piece.piece_type == PieceType::King)
    {
        // are any pieces attacking the king ?
        if can_any_piece_reach_king(king, &pieces_query) {
            println!("check!");
            check.is_check = true;
        }
    }
}
