use std::f32::consts::PI;

use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::behavior::FishBehavior;
use super::shared::{Fish, FishRng, FISH_MAX};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_fish_system)
        .add_systems(Update, spawn_fish_system);
}

#[derive(Resource, Deref)]
pub struct FishMesh(Handle<Mesh>);

#[derive(Resource, Deref)]
pub struct FishMaterials(Vec<Handle<StandardMaterial>>);

#[derive(Resource, Deref, DerefMut)]
pub struct FishTimer(Timer);

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

fn spawn_fish_system(
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
