use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
    window::ExitCondition,
};
use bevy_ratatui::RatatuiPlugins;
use bevy_ratatui_render::RatatuiRenderPlugin;

mod draw;
mod input;
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
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1. / 60.)),
            FrameTimeDiagnosticsPlugin,
            RatatuiPlugins::default(),
            RatatuiRenderPlugin::new("main", (256, 256)),
        ))
        .insert_resource(ClearColor(Color::WHITE))
        .init_resource::<Flags>();

        app.add_plugins((draw::plugin, input::plugin, scene::plugin));
    }
}

#[derive(Resource, Default)]
pub struct Flags {
    debug: bool,
}
