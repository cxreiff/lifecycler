use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_ratatui_camera::RatatuiCamera;

use crate::{general::play_sfx, Flags};

const LIGHT_INTENSITY_DAYTIME: f32 = 500_000.;
const LIGHT_INTENSITY_NIGHTTIME: f32 = 500_000.;

const LIGHT_COLOR_DAYTIME: Color = Color::hsl(190., 0.5, 1.0);
const LIGHT_COLOR_NIGHTTIME: Color = Color::hsl(36., 0.2, 0.5);

const LIGHT_TRANSLATION_DAYTIME: Vec3 = Vec3::new(1.7, 2.5, 5.);
const LIGHT_TRANSLATION_NIGHTTIME: Vec3 = Vec3::new(0., -2.5, 5.);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_camera_system, setup_sfx_system))
        .add_systems(Update, toggle_daylight_system)
        .add_event::<DaylightEvent>();
}

#[derive(Component)]
pub struct Daylight;

#[derive(Event, Default)]
pub struct DaylightEvent;

#[derive(Resource, Deref)]
pub struct ClickOnSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
pub struct ClickOffSound(Handle<AudioSource>);

fn setup_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        RatatuiCamera::default(),
        Bloom::OLD_SCHOOL,
        AtmosphereCamera::default(),
        Msaa::Off,
    ));

    commands.spawn((
        Daylight,
        PointLight {
            intensity: LIGHT_INTENSITY_DAYTIME,
            color: LIGHT_COLOR_DAYTIME,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(LIGHT_TRANSLATION_DAYTIME),
    ));
}

fn setup_sfx_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClickOnSound(
        asset_server.load("embedded://lifecycler/../assets/on.ogg"),
    ));
    commands.insert_resource(ClickOffSound(
        asset_server.load("embedded://lifecycler/../assets/off.ogg"),
    ));
}

fn toggle_daylight_system(
    mut commands: Commands,
    camera: Single<(Entity, Option<&AtmosphereCamera>), With<Camera>>,
    mut light: Single<(&mut PointLight, &mut Transform), With<Daylight>>,
    mut daylight_events: EventReader<DaylightEvent>,
    flags: Res<Flags>,
    on_click: Res<ClickOnSound>,
    off_click: Res<ClickOffSound>,
) {
    for _ in daylight_events.read() {
        let (ref camera_entity, ref atmosphere) = *camera;
        let (ref mut light, ref mut light_transform) = *light;

        if atmosphere.is_some() {
            play_sfx(&mut commands, &off_click, &flags);
            commands.entity(*camera_entity).remove::<AtmosphereCamera>();
            light.intensity = LIGHT_INTENSITY_NIGHTTIME;
            light.color = LIGHT_COLOR_NIGHTTIME;
            light_transform.translation = LIGHT_TRANSLATION_NIGHTTIME;
        } else {
            play_sfx(&mut commands, &on_click, &flags);
            commands
                .entity(*camera_entity)
                .insert(AtmosphereCamera::default());
            light.intensity = LIGHT_INTENSITY_DAYTIME;
            light.color = LIGHT_COLOR_DAYTIME;
            light_transform.translation = LIGHT_TRANSLATION_DAYTIME;
        }
    }
}
