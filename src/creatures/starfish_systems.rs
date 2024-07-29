use std::time::Duration;

use bevy::prelude::*;

use crate::pellets::Pellet;

use super::{
    behavior::{CreatureBehavior, CreatureOperations, CreatureRng},
    starfish_behavior::StarfishOperations,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_starfishes_system)
        .add_systems(
            Update,
            (
                spawn_starfishes_system,
                starfishes_behavior_system,
                starfishes_behavior_change_system,
            ),
        )
        .add_event::<StarfishSpawnEvent>();
}

#[derive(Component)]
pub struct Starfish;

#[derive(Resource, Deref)]
pub struct StarfishScene(Handle<Scene>);

#[derive(Event)]
pub struct StarfishSpawnEvent(pub Vec3, pub f32);

fn setup_starfishes_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_events: EventWriter<StarfishSpawnEvent>,
) {
    let starfish = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset("embedded://lifecycler/../assets/starfish.glb"));
    commands.insert_resource(StarfishScene(starfish));

    spawn_events.send(StarfishSpawnEvent(Vec3::new(-0.3, -1.3, -0.4), 0.2));
}

fn spawn_starfishes_system(
    mut commands: Commands,
    mut spawn_events: EventReader<StarfishSpawnEvent>,
    fish_scene: Res<StarfishScene>,
) {
    for StarfishSpawnEvent(location, size) in spawn_events.read() {
        let transform = Transform::from_translation(*location)
            .with_scale(Vec3::splat(*size))
            .with_rotation(Quat::from_rotation_z(1.));
        let mut behavior = CreatureBehavior::default();
        behavior.timer.set_duration(Duration::from_secs(12));

        commands.spawn((
            Starfish,
            behavior,
            SceneBundle {
                transform,
                scene: fish_scene.clone(),
                ..default()
            },
        ));
    }
}

fn starfishes_behavior_system(
    mut commands: Commands,
    time: Res<Time>,
    mut starfishes: Query<(&mut Transform, &mut CreatureBehavior), With<Starfish>>,
    pellets: Query<(Entity, &mut Transform), (With<Pellet>, Without<CreatureBehavior>)>,
    mut rng: ResMut<CreatureRng>,
) {
    for (mut transform, mut behavior) in starfishes.iter_mut() {
        StarfishOperations::new(&mut transform, &mut behavior).do_behavior(
            &mut commands,
            &mut rng,
            &time,
            &pellets,
        );
    }
}

fn starfishes_behavior_change_system(
    time: Res<Time>,
    mut starfishes: Query<(&mut Transform, &mut CreatureBehavior), With<Starfish>>,
    mut rng: ResMut<CreatureRng>,
) {
    for (mut transform, mut behavior) in starfishes.iter_mut() {
        StarfishOperations::new(&mut transform, &mut behavior).decide_behavior(&time, &mut rng);
    }
}
