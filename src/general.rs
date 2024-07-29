use bevy::prelude::*;

use crate::Flags;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, deferred_despawn_system);
}

#[derive(Component)]
pub struct AttemptDespawn;

fn deferred_despawn_system(
    mut commands: Commands,
    to_despawn_query: Query<Entity, With<AttemptDespawn>>,
) {
    for to_despawn in to_despawn_query.iter() {
        if let Some(mut entity) = commands.get_entity(to_despawn) {
            entity.despawn();
        }
    }
}

pub fn play_sfx(commands: &mut Commands, source: &Handle<AudioSource>, flags: &Flags) {
    if !flags.muted {
        commands.spawn(AudioBundle {
            source: source.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
