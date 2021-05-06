use bevy::{
    app::{AppExit, Events},
    prelude::*,
};
use bevy_mod_picking::*;

use crate::pieces::*;

pub struct PlayerTurn(pub PieceColor);

impl PlayerTurn {
    fn toggle(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}
#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .add_event::<ResetSelectedEvent>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            // .add_system_to_stage(CoreStage::PostUpdate, print_events.system())
            .add_system_to_stage(
                CoreStage::PostUpdate,
                select_square
                    .system()
                    .label("select_square")
                    .after(PickingSystem::Selection),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                move_piece
                    .system()
                    .label("move_piece")
                    .after("select_square"),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                select_piece
                    .system()
                    .label("select_piece")
                    .after("move_piece"),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                despawn_taken_pieces
                    .system()
                    .label("despawn_taken_pieces")
                    .after("move_piece"),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                reset_selected
                    .system()
                    .label("reset_selected")
                    .after("despawn_taken_pieces"),
            );
    }
}
#[derive(PartialEq)]
pub enum SquareColor {
    Dark,
    Light,
}
pub struct Square {
    pub x: u8,
    pub y: u8,
    pub color: SquareColor,
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    // let light_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    // let dark_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    // Spawn 64 squares
    for x in 0..8 {
        for y in 0..8 {
            let color = if (x + y + 1) % 2 == 0 {
                SquareColor::Light
            } else {
                SquareColor::Dark
            };

            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position for alternating pattern
                    material: match color {
                        SquareColor::Dark => materials.add(Color::rgb(0., 0.1, 0.1).into()),
                        SquareColor::Light => materials.add(Color::rgb(1., 0.9, 0.9).into()),
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32, 0., y as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(Square { x, y, color });
        }
    }
}

fn select_square(
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    squares_query: Query<(Entity, &Selection, &Square)>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Populate selected_square resource with the newly selected square
    if let Some((square_entity, _, _)) = squares_query
        .iter()
        .find(|(_, selection, _)| selection.selected())
    {
        selected_square.entity = Some(square_entity);
    } else {
        // Player clicked outside the board, deselect everything
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: ResMut<PlayerTurn>,
    squares_query: Query<(Entity, &Square)>,
    pieces_query: Query<(Entity, &Piece)>,
) {
    if !selected_square.is_changed() {
        return;
    }

    if selected_piece.entity.is_some() {
        return;
    }

    match selected_square.entity {
        None => return,
        Some(selected_square_entity) => {
            if let Ok((_square_entity, square)) = squares_query.get(selected_square_entity) {
                // Select the piece in the currently selected square
                if let Some((piece_entity, _piece)) =
                    pieces_query.iter().find(|(_piece_entity, piece)| {
                        piece.x == square.x && piece.y == square.y && piece.color == turn.0
                    })
                {
                    // piece_entity is now the entity in the same square
                    selected_piece.entity = Some(piece_entity);
                }
            }
        }
    }
}

struct ResetSelectedEvent;

fn reset_selected(
    mut event_reader: EventReader<ResetSelectedEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for _event in event_reader.iter() {
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

fn move_piece(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: ResMut<Events<ResetSelectedEvent>>,
) {
    if !selected_square.is_changed() {
        return;
    }

    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
        let pieces_entity_vec = pieces_query
            .iter_mut()
            .map(|(entity, piece)| (entity, *piece))
            .collect::<Vec<(Entity, Piece)>>();
        // Move the selected piece to the selected square
        let mut piece =
            if let Ok((_piece_entity, piece)) = pieces_query.get_mut(selected_piece_entity) {
                piece
            } else {
                return;
            };

        if piece.is_move_valid((square.x, square.y), pieces_vec) {
            // Check if a piece of the opposite color exists in this square and despawn it
            for (other_entity, other_piece) in pieces_entity_vec {
                if other_piece.x == square.x
                    && other_piece.y == square.y
                    && other_piece.color != piece.color
                {
                    // Mark the piece as taken
                    commands.entity(other_entity).insert(Taken);
                }
            }

            // Move piece
            piece.x = square.x;
            piece.y = square.y;

            // Change turn
            turn.toggle();
        }

        reset_selected_event.send(ResetSelectedEvent);
    }
}

fn despawn_taken_pieces(
    mut commands: Commands,
    mut app_exit_events: ResMut<Events<AppExit>>,
    query: Query<(Entity, &Piece, &Taken)>,
) {
    for (entity, piece, _taken) in query.iter() {
        // If the king is taken, we should exit
        if piece.piece_type == PieceType::King {
            println!(
                "{} won! Thanks for playing!",
                match piece.color {
                    PieceColor::White => "Black",
                    PieceColor::Black => "White",
                }
            );
            app_exit_events.send(AppExit);
        }

        // Despawn piece and children
        commands.entity(entity).despawn_recursive();
    }
}

fn color_squares(
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    squares_query: Query<(
        Entity,
        &Square,
        &Handle<StandardMaterial>,
        &Selection,
        &Hover,
    )>,
) {
    for (entity, square, material_handle, selection, hover) in squares_query.iter() {
        // Get the actual material
        let material = materials.get_mut(material_handle).unwrap();

        // Change the material color
        material.base_color = if hover.hovered() {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.color == SquareColor::Light {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}
