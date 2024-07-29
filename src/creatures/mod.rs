use bevy::prelude::*;

mod behavior;
mod fish_behavior;
mod fish_systems;
mod lifecycle;
mod snail_behavior;
mod snail_systems;
mod starfish_behavior;
mod starfish_systems;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        behavior::plugin,
        fish_systems::plugin,
        lifecycle::plugin,
        snail_systems::plugin,
        starfish_systems::plugin,
    ));
}
