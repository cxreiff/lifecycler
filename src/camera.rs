use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_ratatui_render::RatatuiRenderContext;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera_system);
}

fn setup_camera_system(mut commands: Commands, ratatui_render: Res<RatatuiRenderContext>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 5.).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true,
                target: ratatui_render.target("main").unwrap_or_default(),
                ..default()
            },
            ..default()
        },
        BloomSettings::OLD_SCHOOL,
        AtmosphereCamera::default(),
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0., 3., 5.),
        point_light: PointLight {
            intensity: 1_000_000.,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
