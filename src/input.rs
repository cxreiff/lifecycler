use bevy::prelude::*;
use bevy_ratatui::event::{KeyEvent, MouseEvent};
use bevy_ratatui_camera::{RatatuiCameraLastArea, RatatuiCameraWidget};
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
    mut daylight_event: EventWriter<DaylightEvent>,
) {
    for key_event in ratatui_events.read() {
        match key_event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => match key_event.code {
                KeyCode::Char('q') => {
                    exit.write_default();
                }

                KeyCode::Char('d') => {
                    flags.debug = !flags.debug;
                }

                KeyCode::Char('m') => {
                    flags.muted = !flags.muted;
                }

                KeyCode::Char(' ') => {
                    daylight_event.write_default();
                }

                _ => {}
            },
            _ => {}
        }
    }
}

fn handle_mouse_system(
    mut events: EventReader<MouseEvent>,
    mut pellet_event: EventWriter<PelletEvent>,
    mut drag_threshold: ResMut<DragThreshold>,
    camera: Single<
        (
            &Camera,
            &GlobalTransform,
            &RatatuiCameraWidget,
            &RatatuiCameraLastArea,
        ),
        With<Camera>,
    >,
) {
    for event in events.read() {
        let (camera, camera_transform, camera_widget, last_area) = *camera;

        match event.kind {
            MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Down(MouseButton::Left) => {
                if **drag_threshold == 0 || event.kind == MouseEventKind::Down(MouseButton::Left) {
                    **drag_threshold = DRAGS_PER_EVENT;

                    let ndc = camera_widget.cell_to_ndc(
                        **last_area,
                        IVec2::new(event.column as i32, event.row as i32),
                    );

                    let world_position = camera.ndc_to_world(camera_transform, ndc).unwrap();

                    let viewport_position = camera
                        .world_to_viewport(camera_transform, world_position)
                        .unwrap();

                    let ray = camera
                        .viewport_to_world(camera_transform, viewport_position)
                        .unwrap();

                    let Some(intersect_distance) =
                        ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z))
                    else {
                        return;
                    };

                    let intersect = ray.get_point(intersect_distance);

                    if !(-1.9..1.9).contains(&intersect.x) {
                        return;
                    }

                    let world_transform = Transform::from_translation(intersect);

                    pellet_event.write(PelletEvent(world_transform));
                } else {
                    **drag_threshold -= 1;
                }
            }
            _ => {}
        }
    }
}
