use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
    window::ExitCondition,
};
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_ratatui::RatatuiPlugins;
use bevy_ratatui_render::RatatuiRenderPlugin;

mod draw;
mod input;
mod pellets;
mod scene;

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
                }),
            // DefaultPlugins,
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1. / 90.)),
            FrameTimeDiagnosticsPlugin,
            RatatuiPlugins {
                enable_mouse_capture: true,
                ..default()
            },
            RatatuiRenderPlugin::new("main", (512, 512)),
            // RatatuiRenderPlugin::new("main", (512, 512)).disable(),
            AtmospherePlugin,
        ))
        .insert_resource(Msaa::Off)
        .insert_resource(Flags {
            debug: true,
            msg: "N/A".into(),
        });

        app.add_plugins((draw::plugin, input::plugin, pellets::plugin, scene::plugin));
    }
}

#[derive(Resource, Default)]
pub struct Flags {
    debug: bool,
    msg: String,
}
