use std::f32::consts::PI;

use bevy::prelude::*;

use super::shared::{FISH_AGING_SPEED_SECONDS, FISH_BULK_MAX, FISH_LIFESPAN, FISH_SATIATION_MAX};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_lifecycle_system)
        .add_systems(Update, (age_the_living_system, fish_skeleton_system));
}

#[derive(Component)]
pub struct FishMortality {
    next_age_timer: Timer,
    age: u32,
    pub(super) satiation: u32,
    bulk: u32,
}

impl Default for FishMortality {
    fn default() -> Self {
        Self {
            next_age_timer: Timer::from_seconds(FISH_AGING_SPEED_SECONDS, TimerMode::Repeating),
            age: 0,
            satiation: FISH_SATIATION_MAX / 2,
            bulk: 0,
        }
    }
}

#[derive(Component)]
pub struct FishSkeleton;

#[derive(Resource, Deref)]
pub struct FishSkeletonBundle(SceneBundle);

fn setup_lifecycle_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fish_skeleton = SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("skeleton.glb")),
        ..default()
    };
    commands.insert_resource(FishSkeletonBundle(fish_skeleton));
}

fn age_the_living_system(
    mut commands: Commands,
    time: Res<Time>,
    mut living_query: Query<(Entity, &mut FishMortality, &mut Transform)>,
    fish_skeleton_bundle: Res<FishSkeletonBundle>,
) {
    for (entity, mut mortality, mut transform) in living_query.iter_mut() {
        mortality.next_age_timer.tick(time.delta());

        if mortality.next_age_timer.finished() {
            mortality.next_age_timer.reset();

            mortality.age += 1;
            mortality.satiation -= 1;

            if mortality.satiation > FISH_SATIATION_MAX / 2 && mortality.bulk < FISH_BULK_MAX {
                mortality.bulk += 1;
            }

            transform.scale = Vec3::new(
                0.1 + 0.2 * (mortality.bulk as f32 / FISH_BULK_MAX as f32),
                0.1 + 0.1 * (mortality.bulk as f32 / FISH_BULK_MAX as f32)
                    + 0.25 * (mortality.satiation as f32 / FISH_SATIATION_MAX as f32),
                0.1,
            );

            if mortality.satiation == 0 || mortality.age > FISH_LIFESPAN {
                commands.entity(entity).despawn();
                let mut fish_skeleton = fish_skeleton_bundle.clone();
                fish_skeleton.transform = transform.with_rotation(Quat::from_rotation_x(PI));
                commands.spawn((FishSkeleton, fish_skeleton));
            }
        }
    }
}

fn fish_skeleton_system(
    time: Res<Time>,
    mut commands: Commands,
    mut skeleton_query: Query<(Entity, &mut Transform), With<FishSkeleton>>,
) {
    for (entity, mut transform) in skeleton_query.iter_mut() {
        transform.translation.y -= time.delta_seconds() / 10.;

        if transform.translation.y < -2. {
            commands.entity(entity).despawn();
        }
    }
}
