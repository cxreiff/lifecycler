use std::io;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::utils::error;
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::terminal::RatatuiContext;
use bevy_ratatui_render::RatatuiRenderContext;
use ratatui::style::Stylize;
use ratatui::{
    layout::{Alignment, Rect},
    text::Text,
};

use crate::Flags;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, draw_scene_system.map(error));
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    ratatui_render: Res<RatatuiRenderContext>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        frame.render_widget(ratatui_render.widget("main").unwrap(), frame.size());

        #[cfg(feature = "dev")]
        if flags.debug {
            if let Some(value) = diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
            {
                let position = Rect::new(0, 0, frame.size().width, 1);
                let fps = Text::raw(format!("[fps: {value:.0}]")).alignment(Alignment::Center);

                frame.render_widget(fps, position);
            }
        }

        let position = Rect::new(
            (frame.size().width / 2) - 15,
            frame.size().bottom() - 2,
            30,
            1,
        );
        let keys_info = Text::raw("[q to quit][? for information]")
            .alignment(Alignment::Center)
            .bg(ratatui::style::Color::Black);

        frame.render_widget(keys_info, position);
    })?;

    Ok(())
}
