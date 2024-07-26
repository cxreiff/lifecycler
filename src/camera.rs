use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_ratatui_render::RatatuiRenderContext;

const LIGHT_INTENSITY_DAYTIME: f32 = 800_000.;
const LIGHT_INTENSITY_NIGHTTIME: f32 = 500_000.;

const LIGHT_COLOR_DAYTIME: Color = Color::hsl(190., 0.5, 1.0);
const LIGHT_COLOR_NIGHTTIME: Color = Color::hsl(36., 0.2, 0.5);

const LIGHT_TRANSLATION_DAYTIME: Vec3 = Vec3::new(2., 3., 5.);
const LIGHT_TRANSLATION_NIGHTTIME: Vec3 = Vec3::new(0., -3., 5.);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera_system)
        .add_systems(Update, toggle_daylight_system)
        .add_event::<DaylightEvent>();
}

#[derive(Component)]
pub struct Daylight;

#[derive(Event, Default)]
pub struct DaylightEvent;

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

    commands.spawn((
        Daylight,
        PointLightBundle {
            transform: Transform::from_translation(LIGHT_TRANSLATION_DAYTIME),
            point_light: PointLight {
                intensity: LIGHT_INTENSITY_DAYTIME,
                color: LIGHT_COLOR_DAYTIME,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));
}

fn toggle_daylight_system(
    mut commands: Commands,
    camera_query: Query<(Entity, Option<&AtmosphereCamera>), With<Camera>>,
    mut light_query: Query<(&mut PointLight, &mut Transform), With<Daylight>>,
    mut daylight_events: EventReader<DaylightEvent>,
) {
    for _ in daylight_events.read() {
        let (camera, atmosphere) = camera_query.single();
        let (mut light, mut light_transform) = light_query.single_mut();

        if atmosphere.is_some() {
            commands.entity(camera).remove::<AtmosphereCamera>();
            light.intensity = LIGHT_INTENSITY_NIGHTTIME;
            light.color = LIGHT_COLOR_NIGHTTIME;
            light_transform.translation = LIGHT_TRANSLATION_NIGHTTIME;
        } else {
            commands.entity(camera).insert(AtmosphereCamera::default());
            light.intensity = LIGHT_INTENSITY_DAYTIME;
            light.color = LIGHT_COLOR_DAYTIME;
            light_transform.translation = LIGHT_TRANSLATION_DAYTIME;
        }
    }
}
