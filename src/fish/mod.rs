use bevy::prelude::*;

mod behavior;
mod shared;
mod spawning;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((behavior::plugin, spawning::plugin));
}
