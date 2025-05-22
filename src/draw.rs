use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::RatatuiContext;
use bevy_ratatui_camera::RatatuiCameraWidget;
use ratatui::style::Stylize;
use ratatui::widgets::Widget;
use ratatui::{
    layout::{Alignment, Rect},
    text::Text,
};

use crate::Flags;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, draw_scene_system);
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    mut camera: Single<&mut RatatuiCameraWidget>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
) -> Result {
    ratatui.draw(|frame| {
        camera.render(frame.area(), frame.buffer_mut());

        if flags.debug {
            if let Some(value) = diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
            {
                let _msg = &flags.msg;
                let position = Rect::new(
                    (frame.area().width / 2 + frame.area().width.min(frame.area().height * 2) / 2)
                        .saturating_sub(11 + if flags.muted { 8 } else { 0 }),
                    1 + (frame.area().height * 2).saturating_sub(frame.area().width) / 4,
                    9,
                    1,
                );
                let fps = Text::raw(format!(" fps: {value:.0} "))
                    .alignment(Alignment::Center)
                    .bg(ratatui::style::Color::Black)
                    .fg(ratatui::style::Color::White);

                frame.render_widget(fps, position);
            }
        }

        if flags.muted {
            let position = Rect::new(
                (frame.area().width / 2 + frame.area().width.min(frame.area().height * 2) / 2)
                    .saturating_sub(9),
                1 + (frame.area().height * 2).saturating_sub(frame.area().width) / 4,
                7,
                1,
            );
            let fps = Text::raw(" muted ")
                .alignment(Alignment::Center)
                .bg(ratatui::style::Color::White)
                .fg(ratatui::style::Color::Black);

            frame.render_widget(fps, position);
        }
    })?;

    Ok(())
}
