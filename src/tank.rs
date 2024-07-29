use std::f32::consts::PI;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tank_system);
}

#[derive(Component)]
pub struct Tank;

fn setup_tank_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Tank,
        SceneBundle {
            transform: Transform::from_scale(Vec3::new(2., 0.5, 2.))
                .with_rotation(Quat::from_rotation_x(PI / 2.)),
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("embedded://tank.glb")),
            ..default()
        },
    ));
    commands.spawn(SceneBundle {
        transform: Transform::from_scale(Vec3::new(1.8, 1., 0.5))
            .with_translation(Vec3::new(0., -1.75, 0.)),
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("embedded://gravel.glb")),
        ..default()
    });

    commands.spawn(SceneBundle {
        transform: Transform::from_scale(Vec3::new(2., 0.6, 2.0))
            .with_translation(Vec3::new(0., 0.0, 0.))
            .with_rotation(Quat::from_rotation_x(PI / 2.)),
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("embedded://base.glb")),
        ..default()
    });

    commands.spawn(SceneBundle {
        transform: Transform::from_scale(Vec3::new(2., 2., 0.6))
            .with_translation(Vec3::new(0., 0.0, 0.))
            .with_rotation(Quat::from_rotation_x(0.)),
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("embedded://frame.glb")),
        ..default()
    });

    let coral_bundle = PbrBundle {
        transform: Transform::IDENTITY
            .with_scale(Vec3::splat(0.35))
            .with_rotation(Quat::from_euler(
                EulerRot::ZXY,
                3. * PI / 2.,
                1. * PI / 2.,
                0.,
            )),
        material: materials.add(StandardMaterial::from_color(Color::srgb(0.2, 0.6, 0.5))),
        ..default()
    };

    let mut coral_bundle_1 = coral_bundle.clone();
    coral_bundle_1.transform = coral_bundle_1
        .transform
        .with_translation(Vec3::new(-1.24, -1.8, 0.));
    coral_bundle_1.mesh = asset_server.load(
        (GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        })
        .from_asset("embedded://coral.glb"),
    );
    commands.spawn(coral_bundle_1);

    let mut coral_bundle_2 = coral_bundle.clone();
    coral_bundle_2.transform = coral_bundle_2
        .transform
        .with_translation(Vec3::new(1.2, -1.8, 0.))
        .with_scale(Vec3::splat(0.25));
    coral_bundle_2.mesh = asset_server.load(
        (GltfAssetLabel::Primitive {
            mesh: 1,
            primitive: 0,
        })
        .from_asset("embedded://coral.glb"),
    );
    commands.spawn(coral_bundle_2);

    let rock_bundle = SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("embedded://rocks.glb")),
        ..default()
    };

    let mut rock_bundle_1 = rock_bundle.clone();
    rock_bundle_1.transform = Transform::from_xyz(-1.5, -1.8, -0.5)
        .with_scale(Vec3::splat(0.3))
        .with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0. * PI / 2.,
            3. * PI / 2.,
            0. * PI / 2.,
        ));
    commands.spawn(rock_bundle_1);

    let mut rock_bundle_2 = rock_bundle.clone();
    rock_bundle_2.transform = Transform::from_xyz(-1.7, -1.8, -0.2)
        .with_scale(Vec3::splat(0.15))
        .with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0. * PI / 2.,
            2. * PI / 2.,
            0. * PI / 2.,
        ));
    commands.spawn(rock_bundle_2);

    let mut rock_bundle_2 = rock_bundle.clone();
    rock_bundle_2.transform = Transform::from_xyz(1.6, -1.8, -0.3)
        .with_scale(Vec3::splat(0.2))
        .with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0. * PI / 2.,
            1. * PI / 2.,
            0. * PI / 2.,
        ));
    commands.spawn(rock_bundle_2);

    let mut rock_bundle_3 = rock_bundle.clone();
    rock_bundle_3.transform = Transform::from_xyz(-0.6, -1.8, -0.4)
        .with_scale(Vec3::splat(0.15))
        .with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0. * PI / 2.,
            1. * PI / 2.,
            0. * PI / 2.,
        ));
    commands.spawn(rock_bundle_3);

    let mut rock_bundle_4 = rock_bundle.clone();
    rock_bundle_4.transform = Transform::from_xyz(-0.3, -1.8, -0.4)
        .with_scale(Vec3::splat(0.1))
        .with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0. * PI / 2.,
            1. * PI / 2.,
            0. * PI / 2.,
        ));
    commands.spawn(rock_bundle_4);
}
