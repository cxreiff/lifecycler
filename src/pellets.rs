use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};

use crate::{general::play_sfx, Flags};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_pellets_system, setup_sfx_system))
        .add_systems(
            Update,
            (
                create_pellets_system,
                move_pellets_system,
                perish_perishables_system,
            ),
        )
        .init_resource::<PelletThreshold>()
        .add_event::<PelletEvent>();
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

#[derive(Event, Deref)]
pub struct PelletEvent(pub Transform);

#[derive(Resource, Deref)]
pub struct PelletSound(Handle<AudioSource>);

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
            let base_color = Color::hsl((seeded_rng.next_u32() % 360) as f32, 0.8, 0.8);
            let emissive = base_color.to_linear() * 5.0;

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

fn setup_sfx_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PelletSound(asset_server.load("bubble.ogg")));
}

fn create_pellets_system(
    mut commands: Commands,
    flags: Res<Flags>,
    mut pellet_events: EventReader<PelletEvent>,
    mut pellet_rng: ResMut<PelletRng>,
    pellet_mesh: Res<PelletMesh>,
    pellet_materials: Res<PelletMaterials>,
    pellet_sound: Res<PelletSound>,
) {
    for mouse_position in pellet_events.read() {
        let fall_target = Vec3::new(
            mouse_position.translation.x.clamp(-1.75, 1.75),
            -1.7,
            pellet_rng.next_u32() as f32 / u32::MAX as f32 * 0.75 - 0.25,
        );

        play_sfx(&mut commands, &pellet_sound, &flags);

        commands.spawn((
            Pellet,
            PelletFalling(fall_target),
            PbrBundle {
                transform: **mouse_position,
                mesh: pellet_mesh.clone(),
                material: pellet_materials.choose(&mut pellet_rng.0).unwrap().clone(),
                ..default()
            },
        ));
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
