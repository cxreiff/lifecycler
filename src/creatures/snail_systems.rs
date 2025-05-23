use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::pellets::Pellet;

use super::{
    behavior::{CreatureBehavior, CreatureOperations, CreatureRng},
    snail_behavior::SnailOperations,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_snails_system)
        .add_systems(
            Update,
            (
                spawn_snails_system,
                snails_behavior_system,
                snails_behavior_change_system,
                snails_pellet_detection_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
            ),
        )
        .add_event::<SnailSpawnEvent>();
}

#[derive(Component)]
pub struct Snail;

#[derive(Resource, Deref)]
pub struct SnailScene(Handle<Scene>);

#[derive(Event)]
pub struct SnailSpawnEvent(pub Vec3, pub f32);

fn setup_snails_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_events: EventWriter<SnailSpawnEvent>,
) {
    let snail = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset("embedded://lifecycler/../assets/snail.glb"));
    commands.insert_resource(SnailScene(snail));

    spawn_events.write(SnailSpawnEvent(Vec3::new(-1.4, -1.7, 0.4), 0.1));
    spawn_events.write(SnailSpawnEvent(Vec3::new(1.4, -1.7, -0.1), 0.07));
}

fn spawn_snails_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SnailSpawnEvent>,
    fish_scene: Res<SnailScene>,
) {
    for SnailSpawnEvent(location, size) in spawn_events.read() {
        let transform = Transform::from_translation(*location).with_scale(Vec3::splat(*size));
        let mut behavior = CreatureBehavior::default();
        behavior
            .timer
            .set_duration(behavior.timer.duration() - Duration::from_secs((size * 10.) as u64 % 2));

        commands.spawn((Snail, behavior, SceneRoot(fish_scene.clone()), transform));
    }
}

fn snails_behavior_system(
    mut commands: Commands,
    time: Res<Time>,
    mut snails: Query<(&mut Transform, &mut CreatureBehavior), With<Snail>>,
    pellets: Query<(Entity, &mut Transform), (With<Pellet>, Without<CreatureBehavior>)>,
    mut rng: ResMut<CreatureRng>,
) {
    for (mut transform, mut behavior) in snails.iter_mut() {
        SnailOperations::new(&mut transform, &mut behavior).do_behavior(
            &mut commands,
            &mut rng,
            &time,
            &pellets,
        );
    }
}

fn snails_behavior_change_system(
    time: Res<Time>,
    mut snails: Query<(&mut Transform, &mut CreatureBehavior), With<Snail>>,
    mut rng: ResMut<CreatureRng>,
) {
    for (mut transform, mut behavior) in snails.iter_mut() {
        SnailOperations::new(&mut transform, &mut behavior).decide_behavior(&time, &mut rng);
    }
}

fn snails_pellet_detection_system(
    mut snails: Query<(&mut Transform, &mut CreatureBehavior), With<Snail>>,
    pellets: Query<(Entity, &Transform), (With<Pellet>, Without<CreatureBehavior>)>,
) {
    for (mut transform, mut behavior) in snails.iter_mut() {
        SnailOperations::new(&mut transform, &mut behavior).detect_pellet(&pellets);
    }
}
