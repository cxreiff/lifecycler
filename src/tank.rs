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
        SceneRoot(
            asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("embedded://lifecycler/../assets/tank.glb"),
            ),
        ),
        Transform::from_scale(Vec3::new(2., 0.5, 2.)).with_rotation(Quat::from_rotation_x(PI / 2.)),
    ));
    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("embedded://lifecycler/../assets/gravel.glb"),
        )),
        Transform::from_scale(Vec3::new(1.8, 1., 0.5)).with_translation(Vec3::new(0., -1.75, 0.)),
    ));

    commands.spawn((
        SceneRoot(
            asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("embedded://lifecycler/../assets/base.glb"),
            ),
        ),
        Transform::from_scale(Vec3::new(2., 0.6, 2.0))
            .with_translation(Vec3::new(0., 0.0, 0.))
            .with_rotation(Quat::from_rotation_x(PI / 2.)),
    ));

    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("embedded://lifecycler/../assets/frame.glb"),
        )),
        Transform::from_scale(Vec3::new(2., 2., 0.6))
            .with_translation(Vec3::new(0., 0.0, 0.))
            .with_rotation(Quat::from_rotation_x(0.)),
    ));

    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("embedded://lifecycler/../assets/frame.glb"),
        )),
        Transform::from_scale(Vec3::new(2., 2., 0.6))
            .with_translation(Vec3::new(0., 0.0, 0.))
            .with_rotation(Quat::from_rotation_x(0.)),
    ));

    let coral_transform = Transform::IDENTITY
        .with_scale(Vec3::splat(0.35))
        .with_rotation(Quat::from_euler(
            EulerRot::ZXY,
            3. * PI / 2.,
            1. * PI / 2.,
            0.,
        ));

    let coral_material =
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.2, 0.6, 0.5))));

    commands.spawn((
        coral_transform.with_translation(Vec3::new(-1.24, -1.8, 0.)),
        coral_material.clone(),
        Mesh3d(
            asset_server.load(
                (GltfAssetLabel::Primitive {
                    mesh: 0,
                    primitive: 0,
                })
                .from_asset("embedded://lifecycler/../assets/coral.glb"),
            ),
        ),
    ));

    commands.spawn((
        coral_transform
            .with_translation(Vec3::new(1.2, -1.8, 0.))
            .with_scale(Vec3::splat(0.25)),
        coral_material.clone(),
        Mesh3d(
            asset_server.load(
                (GltfAssetLabel::Primitive {
                    mesh: 1,
                    primitive: 0,
                })
                .from_asset("embedded://lifecycler/../assets/coral.glb"),
            ),
        ),
    ));

    let rock_scene = SceneRoot(
        asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("embedded://lifecycler/../assets/rocks.glb")),
    );

    commands.spawn((
        rock_scene.clone(),
        Transform::from_xyz(-1.5, -1.8, -0.5)
            .with_scale(Vec3::splat(0.3))
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0. * PI / 2.,
                3. * PI / 2.,
                0. * PI / 2.,
            )),
    ));

    commands.spawn((
        rock_scene.clone(),
        Transform::from_xyz(-1.7, -1.8, -0.2)
            .with_scale(Vec3::splat(0.15))
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0. * PI / 2.,
                2. * PI / 2.,
                0. * PI / 2.,
            )),
    ));

    commands.spawn((
        rock_scene.clone(),
        Transform::from_xyz(1.6, -1.8, -0.3)
            .with_scale(Vec3::splat(0.2))
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0. * PI / 2.,
                1. * PI / 2.,
                0. * PI / 2.,
            )),
    ));

    commands.spawn((
        rock_scene.clone(),
        Transform::from_xyz(-0.6, -1.8, -0.4)
            .with_scale(Vec3::splat(0.15))
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0. * PI / 2.,
                1. * PI / 2.,
                0. * PI / 2.,
            )),
    ));

    commands.spawn((
        rock_scene.clone(),
        Transform::from_xyz(-0.3, -1.8, -0.4)
            .with_scale(Vec3::splat(0.1))
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0. * PI / 2.,
                1. * PI / 2.,
                0. * PI / 2.,
            )),
    ));
}
