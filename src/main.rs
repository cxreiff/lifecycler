use bevy::prelude::*;
use lifecycler::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
