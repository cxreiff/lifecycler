use bevy::prelude::*;
use bevy_ratatui::{
    event::{KeyEvent, MouseEvent},
    terminal::RatatuiContext,
};
use crossterm::event::{KeyCode, KeyEventKind, MouseButton, MouseEventKind};

use crate::{camera::DaylightEvent, pellets::PelletEvent, Flags};

const DRAGS_PER_EVENT: u32 = 2;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, (handle_keyboard_system, handle_mouse_system))
        .init_resource::<DragThreshold>();
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DragThreshold(u32);

fn handle_keyboard_system(
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

                KeyCode::Char('d') => {
                    flags.debug = !flags.debug;
                }

                KeyCode::Char('m') => {
                    flags.muted = !flags.muted;
                }

                _ => {}
            },
            _ => {}
        }
    }
}

fn handle_mouse_system(
    ratatui: Res<RatatuiContext>,
    mut events: EventReader<MouseEvent>,
    mut pellet_event: EventWriter<PelletEvent>,
    mut daylight_event: EventWriter<DaylightEvent>,
    mut drag_threshold: ResMut<DragThreshold>,
    camera: Query<&Transform, With<Camera>>,
) {
    for event in events.read() {
        let crossterm::event::MouseEvent {
            kind, column, row, ..
        } = event.0;

        match kind {
            MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Down(MouseButton::Left) => {
                if **drag_threshold == 0 || kind == MouseEventKind::Down(MouseButton::Left) {
                    let size = ratatui.size().unwrap();
                    let camera_transform = camera.single();
                    if let Some(transform) =
                        terminal_coords_to_world_transform(column, row, size, camera_transform)
                    {
                        pellet_event.send(PelletEvent(transform));
                    }
                    **drag_threshold = DRAGS_PER_EVENT;
                } else {
                    **drag_threshold -= 1;
                }
            }
            MouseEventKind::Down(MouseButton::Right) => {
                daylight_event.send_default();
            }
            _ => {}
        }
    }
}

fn terminal_coords_to_world_transform(
    column: u16,
    row: u16,
    terminal_size: ratatui::layout::Rect,
    camera: &Transform,
) -> Option<Transform> {
    let block_width = terminal_size.width;
    let block_height = terminal_size.height * 2;

    let render_column = column as f32 - block_width.saturating_sub(block_height) as f32 / 2.;
    let render_row = (row as f32 - block_height.saturating_sub(block_width) as f32 / 4.) * 2.;

    let x = render_column / block_width.min(block_height) as f32 * 2. - 1.;
    let y = render_row / block_height.min(block_width) as f32 * 2. - 1.;

    if x.abs() > 0.9 || y > 0.9 {
        return None;
    }

    let mut world_coords = *camera * Vec3::new(x * 2.05, -y * 2. + 0.02, 0.);
    world_coords.z = 0.;

    Some(Transform::from_translation(world_coords))
}
