use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use crossterm::event::{KeyCode, KeyEventKind};

use crate::Flags;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, handle_input_system);
}

pub fn handle_input_system(
    mut ratatui_events: EventReader<KeyEvent>,
    mut exit: EventWriter<AppExit>,
    mut flags: ResMut<Flags>,
) {
    for key_event in ratatui_events.read() {
        match key_event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => match key_event.code {
                KeyCode::Char('q') => {
                    exit.send_default();
                }

                #[cfg(feature = "dev")]
                KeyCode::Char('d') => {
                    flags.debug = !flags.debug;
                }

                _ => {}
            },
            _ => {}
        }
    }
}
