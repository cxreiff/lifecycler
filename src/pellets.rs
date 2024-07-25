use bevy::prelude::*;
use bevy_ratatui::{event::MouseEvent, terminal::RatatuiContext};
use crossterm::event::{MouseButton, MouseEventKind};
use rand::seq::SliceRandom;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use ratatui::layout::Rect;

use crate::Flags;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_pellets_system)
        .add_systems(
            Update,
            (
                create_pellets_system,
                move_pellets_system,
                perish_perishables_system,
            ),
        )
        .init_resource::<PelletThreshold>();
}

#[derive(Component)]
pub struct Pellet;

#[derive(Component, Deref)]
pub struct PelletFalling(Vec3);

#[derive(Resource, Deref, DerefMut)]
pub struct PelletRng(ChaCha8Rng);

#[derive(Resource, Deref)]
pub struct PelletMesh(Handle<Mesh>);

#[derive(Resource, Deref)]
pub struct PelletMaterials(Vec<Handle<StandardMaterial>>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct PelletThreshold(u32);

#[derive(Component, Deref, DerefMut)]
pub struct Perishable(Timer);

fn setup_pellets_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_size(Vec3::new(0.03, 0.03, 0.03)));
    commands.insert_resource(PelletMesh(mesh));

    let mut seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
    let pellet_materials = (0..36)
        .map(|_| {
            let base_color = Color::hsl((seeded_rng.next_u32() % 250 + 90) as f32, 1.0, 0.5);
            let emissive = base_color.to_linear() * 10.0;

            materials.add(StandardMaterial {
                base_color,
                emissive,
                ..default()
            })
        })
        .collect();
    commands.insert_resource(PelletMaterials(pellet_materials));
    commands.insert_resource(PelletRng(seeded_rng));
}

fn create_pellets_system(
    mut commands: Commands,
    mut events: EventReader<MouseEvent>,
    ratatui: Res<RatatuiContext>,
    mut pellet_rng: ResMut<PelletRng>,
    pellet_mesh: Res<PelletMesh>,
    pellet_materials: Res<PelletMaterials>,
    mut pellet_threshold: ResMut<PelletThreshold>,
    mut flags: ResMut<Flags>,
    camera: Query<&Transform, With<Camera>>,
) {
    for event in events.read() {
        let crossterm::event::MouseEvent {
            kind, column, row, ..
        } = event.0;
        if let MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Down(MouseButton::Left) =
            kind
        {
            if **pellet_threshold == 0 || kind == MouseEventKind::Down(MouseButton::Left) {
                let size = ratatui.size().unwrap();
                let camera_transform = camera.single();
                if let Some(transform) = terminal_coords_to_world_transform(
                    &mut flags,
                    column,
                    row,
                    size,
                    camera_transform,
                ) {
                    let fall_target = Vec3::new(
                        transform.translation.x.clamp(-1.75, 1.75),
                        -1.7,
                        pellet_rng.next_u32() as f32 / u32::MAX as f32 * 0.75 - 0.25,
                    );
                    commands.spawn((
                        Pellet,
                        PelletFalling(fall_target),
                        PbrBundle {
                            transform,
                            mesh: pellet_mesh.clone(),
                            material: pellet_materials.choose(&mut pellet_rng.0).unwrap().clone(),
                            ..default()
                        },
                    ));
                }
                **pellet_threshold = 1;
            } else {
                **pellet_threshold -= 1;
            }
        }
    }
}

fn move_pellets_system(
    mut commands: Commands,
    mut pellets: Query<(Entity, &mut Transform, &PelletFalling)>,
    time: Res<Time>,
) {
    for (id, mut pellet_transform, PelletFalling(fall_target)) in &mut pellets {
        pellet_transform.translation = pellet_transform
            .translation
            .move_towards(*fall_target, time.delta_seconds() * 0.3);

        pellet_transform.translation.x +=
            (time.elapsed_seconds() + (fall_target.x * 16.) % 3.).sin() / 800.;
        pellet_transform.translation.x = pellet_transform.translation.x.clamp(-1.8, 1.8);

        if pellet_transform.translation.distance(*fall_target) < 0.003 {
            let mut entity = commands.entity(id);
            entity.remove::<PelletFalling>();
            entity.insert(Perishable(Timer::from_seconds(20., TimerMode::Once)));
        }
    }
}

fn perish_perishables_system(
    mut commands: Commands,
    time: Res<Time>,
    mut perishables: Query<(Entity, &mut Perishable)>,
) {
    let delta = time.delta();
    for (id, mut timer) in &mut perishables {
        timer.tick(delta);
        if timer.finished() {
            commands.entity(id).despawn();
        }
    }
}

fn terminal_coords_to_world_transform(
    flags: &mut ResMut<Flags>,
    column: u16,
    row: u16,
    terminal_size: Rect,
    camera: &Transform,
) -> Option<Transform> {
    let block_width = terminal_size.width;
    let block_height = terminal_size.height * 2;

    let render_column = column as f32 - block_width.saturating_sub(block_height) as f32 / 2.;
    let render_row = (row as f32 - block_height.saturating_sub(block_width) as f32 / 4.) * 2.;

    let x = render_column / block_width.min(block_height) as f32 * 2. - 1.;
    let y = render_row / block_height.min(block_width) as f32 * 2. - 1.;

    if x.abs() > 0.9 || y > 0.9 {
        return None;
    }

    let mut world_coords = *camera * Vec3::new(x * 2.05, -y * 2. + 0.02, 0.);
    world_coords.z = 0.;
    flags.msg = format!("{world_coords:?}");

    Some(Transform::from_translation(world_coords))
}
