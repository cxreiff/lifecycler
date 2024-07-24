use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::HashSet;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::pellets::Pellet;

const FISH_MAX: usize = 5;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_fish_system).add_systems(
        Update,
        (
            spawn_fishes_system,
            fish_behavior_system,
            fish_behavior_change_system,
            fish_pellet_detection_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
        ),
    );
}

#[derive(Component)]
pub struct Fish;

#[derive(Resource, Deref, DerefMut)]
pub struct FishRng(ChaCha8Rng);

#[derive(Resource, Deref)]
pub struct FishMesh(Handle<Mesh>);

#[derive(Resource, Deref)]
pub struct FishMaterials(Vec<Handle<StandardMaterial>>);

#[derive(Resource, Deref, DerefMut)]
pub struct FishTimer(Timer);

pub enum FishBehaviorVariant {
    Idle,
    SwimRight,
    SwimLeft,
    SeekPoint(Vec3),
    SeekPellet(Entity),
}

#[derive(Component)]
pub struct FishBehavior {
    timer: Timer,
    variant: FishBehaviorVariant,
}

impl Default for FishBehavior {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
            variant: FishBehaviorVariant::Idle,
        }
    }
}

fn setup_fish_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let fish_mesh = asset_server.load(
        (GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        })
        .from_asset("fish.glb"),
    );
    commands.insert_resource(FishMesh(fish_mesh));

    let mut seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
    let fish_materials = (0..36)
        .map(|_| {
            let base_color = Color::hsl((seeded_rng.next_u32() % 360) as f32, 0.4, 0.3);
            let emissive = base_color.to_linear() * 0.1;

            materials.add(StandardMaterial {
                base_color,
                emissive,
                ..default()
            })
        })
        .collect();
    commands.insert_resource(FishMaterials(fish_materials));
    commands.insert_resource(FishRng(seeded_rng));

    let fish_timer = Timer::from_seconds(0.5, TimerMode::Repeating);
    commands.insert_resource(FishTimer(fish_timer));
}

fn spawn_fishes_system(
    mut commands: Commands,
    time: Res<Time>,
    mut fish_timer: ResMut<FishTimer>,
    fish_mesh: Res<FishMesh>,
    fish_materials: Res<FishMaterials>,
    mut fish_rng: ResMut<FishRng>,
    fishes: Query<Entity, With<Fish>>,
) {
    fish_timer.tick(time.delta());

    let fish_count = fishes.iter().len();

    if fish_timer.just_finished() && fish_count < FISH_MAX {
        let mut transform = Transform::default()
            .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2., PI, 0.))
            .with_scale(Vec3::new(0.2, 0.1, 0.15));
        transform.translation += Cuboid::new(1.5, 1.5, 0.8).sample_interior(&mut fish_rng.0);

        commands.spawn((
            Fish,
            FishBehavior::default(),
            PbrBundle {
                transform,
                mesh: fish_mesh.clone(),
                material: fish_materials.choose(&mut fish_rng.0).unwrap().clone(),
                ..default()
            },
        ));
    }
}

fn fish_behavior_system(
    mut commands: Commands,
    time: Res<Time>,
    mut fishes: Query<(&mut Transform, &mut FishBehavior)>,
    pellets: Query<&Transform, (With<Pellet>, Without<FishBehavior>)>,
    mut fish_rng: ResMut<FishRng>,
) {
    let mut despawn_set = HashSet::new();

    for (mut transform, mut behavior) in fishes.iter_mut() {
        match behavior.variant {
            FishBehaviorVariant::Idle => {
                transform.translation.y += time.elapsed_seconds().sin() / 3000.;
            }
            FishBehaviorVariant::SwimRight => {
                transform.translation.x += time.delta_seconds() / 10.;
                if transform.translation.x > 1.4 {
                    *transform =
                        transform.with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.));
                    behavior.variant = FishBehaviorVariant::SwimLeft;
                }
            }
            FishBehaviorVariant::SwimLeft => {
                transform.translation.x -= time.delta_seconds() / 10.;
                if transform.translation.x < -1.4 {
                    *transform =
                        transform.with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2., PI, 0.));
                    behavior.variant = FishBehaviorVariant::SwimRight;
                }
            }
            FishBehaviorVariant::SeekPoint(target) => {
                if transform.translation.x < target.x {
                    *transform =
                        transform.with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2., PI, 0.));
                } else {
                    *transform =
                        transform.with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.));
                }

                transform.translation = transform
                    .translation
                    .move_towards(target, time.delta_seconds() / 10.);

                if transform.translation.distance(target) < 0.1 {
                    behavior.variant = FishBehaviorVariant::Idle;
                    behavior.timer.reset();
                }
            }
            FishBehaviorVariant::SeekPellet(pellet_id) => {
                if let Ok(pellet_transform) = pellets.get(pellet_id) {
                    if transform.translation.x < pellet_transform.translation.x {
                        *transform = transform.with_rotation(Quat::from_euler(
                            EulerRot::XYZ,
                            PI / 2.,
                            PI,
                            0.,
                        ));
                    } else {
                        *transform = transform.with_rotation(Quat::from_euler(
                            EulerRot::XYZ,
                            PI / 2.,
                            0.,
                            0.,
                        ));
                    }

                    transform.translation = transform
                        .translation
                        .move_towards(pellet_transform.translation, time.delta_seconds() * 0.5);

                    transform.translation = transform
                        .translation
                        .clamp(Vec3::new(-1.7, -1.65, -0.4), Vec3::new(1.7, 1.6, 0.4));

                    if transform.translation.distance(pellet_transform.translation) < 0.17 {
                        despawn_set.insert(pellet_id);
                    }
                } else {
                    let target = Cuboid::new(1.5, 1.5, 0.8).sample_interior(&mut fish_rng.0);
                    behavior.variant = FishBehaviorVariant::SeekPoint(target);
                    behavior.timer.reset();
                }
            }
        }
    }

    for pellet_id in despawn_set.iter() {
        if let Some(mut entity) = commands.get_entity(*pellet_id) {
            entity.despawn();
        }
    }
}

fn fish_behavior_change_system(
    time: Res<Time>,
    mut fishes: Query<(&Transform, &mut FishBehavior)>,
) {
    for (transform, mut behavior) in fishes.iter_mut() {
        behavior.timer.tick(time.delta());

        if behavior.timer.just_finished() {
            behavior.variant = match behavior.variant {
                FishBehaviorVariant::Idle => {
                    if transform.rotation.y == 0. {
                        FishBehaviorVariant::SwimLeft
                    } else {
                        FishBehaviorVariant::SwimRight
                    }
                }
                FishBehaviorVariant::SwimRight => FishBehaviorVariant::Idle,
                FishBehaviorVariant::SwimLeft => FishBehaviorVariant::Idle,
                FishBehaviorVariant::SeekPoint(target) => FishBehaviorVariant::SeekPoint(target),
                FishBehaviorVariant::SeekPellet(entity) => FishBehaviorVariant::SeekPellet(entity),
            }
        }
    }
}

fn fish_pellet_detection_system(
    mut fishes: Query<(&Transform, &mut FishBehavior)>,
    pellets: Query<(Entity, &Transform), With<Pellet>>,
) {
    for (transform, mut behavior) in fishes.iter_mut() {
        let (closest_pellet_id, closest_dist) = pellets.iter().fold(
            (None, f32::MAX),
            |(closest_pellet_id, closest_dist), (pellet_id, pellet_transform)| {
                let dist = transform
                    .translation
                    .xy()
                    .distance(pellet_transform.translation.xy());
                if dist < closest_dist {
                    (Some(pellet_id), dist)
                } else {
                    (closest_pellet_id, closest_dist)
                }
            },
        );

        if let Some(closest_pellet_id) = closest_pellet_id {
            if closest_dist < 0.8 {
                behavior.variant = FishBehaviorVariant::SeekPellet(closest_pellet_id);
            }
        }
    }
}
