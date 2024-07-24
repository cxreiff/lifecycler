use bevy::prelude::*;
use bevy_atmosphere::{
    model::AtmosphereModel,
    prelude::{AtmosphereMut, Nishita},
};
use light_consts::lux::AMBIENT_DAYLIGHT;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_sky_system)
        .add_systems(Update, daylight_cycle_system)
        .insert_resource(AtmosphereModel::default())
        .insert_resource(CycleTimer(Timer::new(
            bevy::utils::Duration::from_millis(50),
            TimerMode::Repeating,
        )));
}

#[derive(Component)]
pub struct Sun;

#[derive(Resource, Deref, DerefMut)]
struct CycleTimer(Timer);

fn setup_sky_system(mut commands: Commands) {
    commands.spawn((Sun, DirectionalLightBundle::default()));
}

fn daylight_cycle_system(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.tick(time.delta());

    if timer.finished() {
        let t = time.elapsed_seconds_wrapped() / 24.0;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
        }
    }
}
