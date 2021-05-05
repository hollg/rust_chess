use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .add_startup_system(create_board.system())
            // .add_system_to_stage(CoreStage::PostUpdate, print_events.system())
            .add_system_to_stage(CoreStage::PostUpdate, select_square.system());
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

pub fn create_board(
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

pub fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        info!("{:?}", event);
    }
}

fn select_square(
    mut selected_square: ResMut<SelectedSquare>,
    mut query: Query<(Entity, &mut PickableBundle)>,
) {
    for (entity, bundle) in query.iter_mut() {
        if bundle.selection.selected() {
            selected_square.entity = Some(entity);
        }
    }
}
