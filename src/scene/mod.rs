use bevy::prelude::*;

mod camera;
mod tank;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, tank::plugin));
}
