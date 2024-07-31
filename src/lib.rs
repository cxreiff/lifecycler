use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, diagnostic::FrameTimeDiagnosticsPlugin, log::LogPlugin, prelude::*,
    window::ExitCondition,
};
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_hanabi::HanabiPlugin;
use bevy_ratatui::RatatuiPlugins;
use bevy_ratatui_render::RatatuiRenderPlugin;

mod assets;
mod bubbles;
mod camera;
mod creatures;
mod draw;
mod general;
mod input;
mod pellets;
mod tank;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    close_when_requested: false,
                })
                .disable::<LogPlugin>(),
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1. / 90.)),
            FrameTimeDiagnosticsPlugin,
            RatatuiPlugins {
                enable_mouse_capture: true,
                ..default()
            },
            RatatuiRenderPlugin::new("main", (512, 512)),
            AtmospherePlugin,
            HanabiPlugin,
        ))
        .insert_resource(Msaa::Off)
        .init_resource::<Flags>();

        app.add_plugins((
            assets::plugin,
            bubbles::plugin,
            camera::plugin,
            draw::plugin,
            creatures::plugin,
            general::plugin,
            input::plugin,
            pellets::plugin,
            tank::plugin,
        ));
    }
}

#[derive(Resource, Default)]
pub struct Flags {
    debug: bool,
    muted: bool,
    msg: String,
}
