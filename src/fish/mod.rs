use bevy::prelude::*;

mod behavior;
mod lifecycle;
mod shared;
mod spawning;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((behavior::plugin, lifecycle::plugin, spawning::plugin));
}
