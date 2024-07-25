use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_lifecycle_system)
        .add_systems(
            Update,
            (
                update_age_system,
                update_bulk_system,
                update_satiation_system,
            ),
        );
}

#[derive(Component, Deref, DerefMut)]
pub struct Age(f32);

#[derive(Component, Deref, DerefMut)]
pub struct Bulk(u32);

#[derive(Component, Deref, DerefMut)]
pub struct Satiation(u32);

fn setup_lifecycle_system() {
    //
}

fn update_age_system(time: Res<Time>, mut age_query: Query<&mut Age>) {
    for mut age in age_query.iter_mut() {
        **age += time.delta_seconds();
    }
}

fn update_bulk_system() {
    //
}

fn update_satiation_system() {
    //
}
