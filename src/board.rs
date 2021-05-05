use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::pieces::*;

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
            .add_startup_system(create_board.system())
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
                select_piece.system().after("select_square"),
            );
    }
}
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
    for i in 0..8 {
        for j in 0..8 {
            let color = if (i + j + 1) % 2 == 0 {
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
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(Square { x: i, y: j, color });
        }
    }
}

fn select_square(
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    squares_query: Query<(Entity, &Selection, &Square)>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Populate selected_square resource with the newly selected square
    if let Some((square_entity, _selection, square)) = squares_query
        .iter()
        .find(|(_square_entity, selection, _square)| selection.selected())
    {
        selected_square.entity = Some(square_entity);

        // If there is already a selected piece, move it to the selected square
        // TODO: figure out why the move only happens after 1 more click
        if let Some(selected_piece_entity) = selected_piece.entity {
            println!("There is a selected piece");
            if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
                println!("About to move the piece");
                piece.x = square.x;
                piece.y = square.y;
            }

            // Then deselect everything
            selected_square.entity = None;
            selected_piece.entity = None;
        }
    } else {
        // Player clicked outside the board, deselect everything
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
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
                if let Some((piece_entity, _piece)) = pieces_query
                    .iter()
                    .find(|(_piece_entity, piece)| piece.x == square.x && piece.y == square.y)
                {
                    // piece_entity is now the entity in the same square
                    selected_piece.entity = Some(piece_entity);
                }
            }
        }
    }
}
