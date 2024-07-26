use std::f32::consts::PI;
use std::time::Duration;

use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand_chacha::ChaCha8Rng;

use crate::general::AttemptDespawn;
use crate::pellets::Pellet;

use super::lifecycle::FishMortality;
use super::shared::{FishRng, FISH_SATIATION_MAX};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            fish_behavior_system,
            fish_behavior_change_system,
            fish_pellet_detection_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
        ),
    );
}

pub enum FishBehaviorVariant {
    Debut,
    Idle,
    SwimRight,
    SwimLeft,
    SeekPoint(Vec3),
    SeekPellet(Entity),
}

#[derive(Component)]
pub struct FishBehavior {
    timer: Timer,
    variant: FishBehaviorVariant,
}

impl Default for FishBehavior {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
            variant: FishBehaviorVariant::Debut,
        }
    }
}

pub struct FishOperations<'a> {
    transform: &'a mut Transform,
    behavior: &'a mut FishBehavior,
    mortality: &'a mut FishMortality,
}

impl<'a> FishOperations<'a> {
    fn new(
        transform: &'a mut Transform,
        behavior: &'a mut FishBehavior,
        mortality: &'a mut FishMortality,
    ) -> Self {
        Self {
            transform,
            behavior,
            mortality,
        }
    }

    fn start_idle(&mut self) {
        self.behavior.variant = FishBehaviorVariant::Idle;
        self.behavior.timer.reset();
    }

    fn start_swim_right(&mut self) {
        self.face_right();
        self.behavior.variant = FishBehaviorVariant::SwimRight;
        self.behavior.timer.reset();
    }

    fn start_swim_left(&mut self) {
        self.face_left();
        self.behavior.variant = FishBehaviorVariant::SwimLeft;
        self.behavior.timer.reset();
    }

    fn start_seek_point(&mut self, rng: &mut ChaCha8Rng) {
        let point = Cuboid::new(1.5, 1.5, 0.8).sample_interior(rng);

        if self.transform.translation.x < point.x {
            self.face_right();
        } else {
            self.face_left();
        }

        self.behavior.variant = FishBehaviorVariant::SeekPoint(point);
        self.behavior.timer.reset();
    }

    fn start_seek_pellet(&mut self, pellet_id: Entity) {
        self.behavior.variant = FishBehaviorVariant::SeekPellet(pellet_id);
        self.behavior.timer.reset();
    }

    fn behavior_debut(&mut self, time: &Time, rng: &mut FishRng) {
        self.transform.translation.y += time.delta_seconds() / 10.;

        if self.transform.translation.y > -0.5 {
            self.start_seek_point(rng)
        }
    }

    fn behavior_idle(&mut self, time: &Time) {
        self.transform.translation.y += time.elapsed_seconds().sin() / 3000.;
    }

    fn behavior_swim_right(&mut self, time: &Time) {
        self.transform.translation.x += time.delta_seconds() / 10.;

        if self.transform.translation.x > 1.4 {
            self.start_swim_left();
        }
    }

    fn behavior_swim_left(&mut self, time: &Time) {
        self.transform.translation.x -= time.delta_seconds() / 10.;

        if self.transform.translation.x < -1.4 {
            self.start_swim_right();
        }
    }

    fn behavior_seek_point(&mut self, time: &Time, target: Vec3) {
        self.transform.translation = self
            .transform
            .translation
            .move_towards(target, time.delta_seconds() / 10.);

        if self.transform.translation.distance(target) < 0.1 {
            self.start_idle();
        }
    }

    fn behavior_seek_pellet(
        &mut self,
        time: &Time,
        rng: &mut FishRng,
        pellet: Result<(Entity, &Transform), QueryEntityError>,
        commands: &mut Commands,
    ) {
        if let Ok((pellet_entity, pellet_transform)) = pellet {
            if self.mortality.satiation >= FISH_SATIATION_MAX {
                self.start_seek_point(rng);
                return;
            }

            if self.transform.translation.x < pellet_transform.translation.x {
                self.face_right();
            } else {
                self.face_left();
            }

            self.transform.translation = self
                .transform
                .translation
                .move_towards(pellet_transform.translation, time.delta_seconds() * 0.5);

            if self
                .transform
                .translation
                .distance(pellet_transform.translation)
                < 0.17
            {
                if let Some(mut entity) = commands.get_entity(pellet_entity) {
                    self.mortality.satiation += 10;
                    entity.insert(AttemptDespawn);
                }
            }
        } else {
            self.start_seek_point(rng);
        }
    }

    fn face_right(&mut self) {
        *self.transform =
            self.transform
                .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2., PI, 0.));
    }

    fn face_left(&mut self) {
        *self.transform =
            self.transform
                .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.));
    }

    fn clamp(&mut self) {
        self.transform.translation = self
            .transform
            .translation
            .clamp(Vec3::new(-1.7, -1.65, -0.4), Vec3::new(1.7, 1.6, 0.4));
    }
}

fn fish_behavior_system(
    mut commands: Commands,
    time: Res<Time>,
    mut fishes: Query<(&mut Transform, &mut FishBehavior, &mut FishMortality)>,
    pellets: Query<(Entity, &mut Transform), (With<Pellet>, Without<FishBehavior>)>,
    mut rng: ResMut<FishRng>,
) {
    for (mut transform, mut behavior, mut mortality) in fishes.iter_mut() {
        let mut fish = FishOperations::new(&mut transform, &mut behavior, &mut mortality);

        match fish.behavior.variant {
            FishBehaviorVariant::Debut => fish.behavior_debut(&time, &mut rng),
            FishBehaviorVariant::Idle => fish.behavior_idle(&time),
            FishBehaviorVariant::SwimRight => fish.behavior_swim_right(&time),
            FishBehaviorVariant::SwimLeft => fish.behavior_swim_left(&time),
            FishBehaviorVariant::SeekPoint(point) => fish.behavior_seek_point(&time, point),
            FishBehaviorVariant::SeekPellet(pellet_id) => {
                fish.behavior_seek_pellet(&time, &mut rng, pellets.get(pellet_id), &mut commands)
            }
        }

        fish.clamp();
    }
}

fn fish_behavior_change_system(
    time: Res<Time>,
    mut fishes: Query<(&mut Transform, &mut FishBehavior, &mut FishMortality)>,
) {
    for (mut transform, mut behavior, mut mortality) in fishes.iter_mut() {
        let mut fish = FishOperations::new(&mut transform, &mut behavior, &mut mortality);

        fish.behavior.timer.tick(time.delta());

        if fish.behavior.timer.just_finished() {
            match fish.behavior.variant {
                FishBehaviorVariant::Idle => {
                    if fish.transform.rotation.y == 0. {
                        fish.start_swim_left();
                    } else {
                        fish.start_swim_right();
                    }
                }
                FishBehaviorVariant::SwimRight => fish.start_idle(),
                FishBehaviorVariant::SwimLeft => fish.start_idle(),
                _ => {}
            }
        }
    }
}

fn fish_pellet_detection_system(
    mut fishes: Query<(&mut Transform, &mut FishBehavior, &mut FishMortality)>,
    pellets: Query<(Entity, &Transform), (With<Pellet>, Without<FishBehavior>)>,
) {
    for (mut transform, mut behavior, mut mortality) in fishes.iter_mut() {
        let mut fish = FishOperations::new(&mut transform, &mut behavior, &mut mortality);

        let (closest_pellet_id, closest_dist) = pellets.iter().fold(
            (None, f32::MAX),
            |(closest_pellet_id, closest_dist), (pellet_id, pellet_transform)| {
                let dist = fish
                    .transform
                    .translation
                    .xy()
                    .distance(pellet_transform.translation.xy());
                if dist < closest_dist {
                    (Some(pellet_id), dist)
                } else {
                    (closest_pellet_id, closest_dist)
                }
            },
        );

        if let Some(closest_pellet_id) = closest_pellet_id {
            if closest_dist < 0.8 {
                fish.start_seek_pellet(closest_pellet_id);
            }
        }
    }
}
