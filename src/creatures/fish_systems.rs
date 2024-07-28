use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::seq::SliceRandom;
use rand::RngCore;

use crate::pellets::Pellet;

use super::behavior::{CreatureBehavior, CreatureOperations, CreatureRng};
use super::fish_behavior::{FishOperations, FISH_MAX, FISH_SPAWN_INTERVAL_SECONDS};
use super::lifecycle::{FishMortality, FishSkeleton};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_fish_system)
        .add_systems(
            Update,
            (
                populate_fish_system
                    .run_if(on_timer(Duration::from_secs(FISH_SPAWN_INTERVAL_SECONDS))),
                fish_spawn_system,
                fish_behavior_system,
                fish_behavior_change_system,
                fish_pellet_detection_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
            ),
        )
        .add_event::<FishSpawnEvent>();
}

#[derive(Component)]
pub struct Fish;

#[derive(Resource, Deref)]
pub struct FishMesh(Handle<Mesh>);

#[derive(Resource, Deref)]
pub struct FishMaterials(Vec<Handle<StandardMaterial>>);

#[derive(Event, Deref)]
pub struct FishSpawnEvent(pub Vec3);

fn setup_fish_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_events: EventWriter<FishSpawnEvent>,
    mut rng: ResMut<CreatureRng>,
) {
    let fish_mesh = asset_server.load(
        (GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        })
        .from_asset("embedded://fish.glb"),
    );
    commands.insert_resource(FishMesh(fish_mesh));

    let fish_materials = (0..36)
        .map(|_| {
            let base_color = Color::hsl(((rng.next_u32() % 160 + 200) % 360) as f32, 0.4, 0.3);
            let emissive = base_color.to_linear() * 0.3;

            materials.add(StandardMaterial {
                base_color,
                emissive,
                perceptual_roughness: 1.,
                ..default()
            })
        })
        .collect();

    commands.insert_resource(FishMaterials(fish_materials));

    spawn_events.send(FishSpawnEvent(
        FishOperations::valid_random_point(&mut rng).with_y(-1.7),
    ));
}

fn populate_fish_system(
    mut rng: ResMut<CreatureRng>,
    fishes: Query<Entity, With<Fish>>,
    skeletons: Query<Entity, With<FishSkeleton>>,
    mut spawn_events: EventWriter<FishSpawnEvent>,
) {
    let fish_count = fishes.iter().len() + skeletons.iter().len();

    if fish_count < FISH_MAX {
        spawn_events.send(FishSpawnEvent(
            FishOperations::valid_random_point(&mut rng).with_y(-1.7),
        ));
    }
}

fn fish_spawn_system(
    mut commands: Commands,
    mut spawn_events: EventReader<FishSpawnEvent>,
    fish_mesh: Res<FishMesh>,
    fish_materials: Res<FishMaterials>,
    mut rng: ResMut<CreatureRng>,
) {
    for spawn_location in spawn_events.read() {
        let transform = Transform::from_translation(**spawn_location)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2., PI, 0.))
            .with_scale(Vec3::new(0.1, 0.1, 0.1));

        commands.spawn((
            Fish,
            CreatureBehavior::default(),
            FishMortality::new(&mut rng),
            PbrBundle {
                transform,
                mesh: fish_mesh.clone(),
                material: fish_materials.choose(&mut rng.0).unwrap().clone(),
                ..default()
            },
        ));
    }
}

fn fish_behavior_system(
    mut commands: Commands,
    time: Res<Time>,
    mut fishes: Query<(&mut Transform, &mut CreatureBehavior, &mut FishMortality), With<Fish>>,
    pellets: Query<(Entity, &mut Transform), (With<Pellet>, Without<CreatureBehavior>)>,
    mut rng: ResMut<CreatureRng>,
) {
    for (mut transform, mut behavior, mut mortality) in fishes.iter_mut() {
        FishOperations::new(&mut transform, &mut behavior, &mut mortality).do_behavior(
            &mut commands,
            &mut rng,
            &time,
            &pellets,
        );
    }
}

fn fish_behavior_change_system(
    time: Res<Time>,
    mut fishes: Query<(&mut Transform, &mut CreatureBehavior, &mut FishMortality), With<Fish>>,
) {
    for (mut transform, mut behavior, mut mortality) in fishes.iter_mut() {
        FishOperations::new(&mut transform, &mut behavior, &mut mortality).decide_behavior(&time);
    }
}

fn fish_pellet_detection_system(
    mut fishes: Query<(&mut Transform, &mut CreatureBehavior, &mut FishMortality), With<Fish>>,
    pellets: Query<(Entity, &Transform), (With<Pellet>, Without<CreatureBehavior>)>,
) {
    for (mut transform, mut behavior, mut mortality) in fishes.iter_mut() {
        FishOperations::new(&mut transform, &mut behavior, &mut mortality).detect_pellet(&pellets);
    }
}
