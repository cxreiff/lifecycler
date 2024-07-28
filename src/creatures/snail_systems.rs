use std::{f32::consts::PI, time::Duration};

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

#[derive(Event, Deref)]
pub struct SnailSpawnEvent(pub Vec3);

fn setup_snails_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_events: EventWriter<SnailSpawnEvent>,
) {
    let snail = asset_server.load(GltfAssetLabel::Scene(0).from_asset("embedded://snail.glb"));
    commands.insert_resource(SnailScene(snail));

    spawn_events.send(SnailSpawnEvent(Vec3::new(-1.4, -1.7, 0.4)));
}

fn spawn_snails_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SnailSpawnEvent>,
    fish_scene: Res<SnailScene>,
) {
    for spawn_location in spawn_events.read() {
        let transform = Transform::from_translation(**spawn_location)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., 3. * PI / 2., 0.))
            .with_scale(Vec3::new(0.09, 0.09, 0.09));

        commands.spawn((
            Snail,
            CreatureBehavior::default(),
            SceneBundle {
                transform,
                scene: fish_scene.clone(),
                ..default()
            },
        ));
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
) {
    for (mut transform, mut behavior) in snails.iter_mut() {
        SnailOperations::new(&mut transform, &mut behavior).decide_behavior(&time);
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
