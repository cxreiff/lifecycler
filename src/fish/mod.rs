use bevy::prelude::*;

pub use shared::Fish;

mod behavior;
mod shared;
mod spawning;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((behavior::plugin, spawning::plugin));
}
