use std::f32::consts::PI;

use bevy::{ecs::query::QueryEntityError, prelude::*};

use crate::general::AttemptDespawn;

use super::{
    behavior::{CreatureBehavior, CreatureOperations, CreatureRng},
    lifecycle::FishMortality,
};

pub(super) const FISH_MAX: usize = 12;
pub(super) const FISH_SPAWN_INTERVAL_SECONDS: u64 = 4;
pub(super) const FISH_AGING_INTERVAL_SECONDS: f32 = 10.;
pub(super) const FISH_SATIATION_MAX: u32 = 64;
pub(super) const FISH_AVERAGE_LONGEVITY: u32 = 64;
pub(super) const FISH_BULK_MAX: u32 = 32;

pub(super) struct FishOperations<'a> {
    transform: &'a mut Transform,
    behavior: &'a mut CreatureBehavior,
    mortality: &'a mut FishMortality,
}

impl<'a> FishOperations<'a> {
    pub(super) fn new(
        transform: &'a mut Transform,
        behavior: &'a mut CreatureBehavior,
        mortality: &'a mut FishMortality,
    ) -> Self {
        Self {
            transform,
            behavior,
            mortality,
        }
    }
}

impl<'a> CreatureOperations for FishOperations<'a> {
    fn behavior_debut(&mut self, time: &Time, rng: &mut CreatureRng) {
        self.transform.translation.y += time.delta_seconds() * Self::base_speed();

        if self.transform.translation.y > -0.5 {
            self.start_seek_point(rng);
        }
    }

    fn behavior_idle(&mut self, time: &Time) {
        self.transform.translation.y += time.elapsed_seconds().sin() / 3000.;
    }

    fn behavior_seek_pellet(
        &mut self,
        time: &Time,
        rng: &mut CreatureRng,
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

            self.transform.translation = self.transform.translation.move_towards(
                pellet_transform.translation,
                time.delta_seconds() * Self::base_speed() * 4.,
            );

            if self
                .transform
                .translation
                .distance(pellet_transform.translation)
                < 0.1
            {
                if let Some(mut entity) = commands.get_entity(pellet_entity) {
                    self.mortality.satiation += 5;
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

    fn rank_pellet(&mut self, pellet_transform: &Transform) -> f32 {
        self.transform()
            .translation
            .xy()
            .distance(pellet_transform.translation.xy())
    }

    fn check_pellet(&mut self, rank: f32) -> bool {
        rank < 0.8
    }

    fn behavior(&mut self) -> &mut CreatureBehavior {
        self.behavior
    }

    fn transform(&mut self) -> &mut Transform {
        self.transform
    }

    fn base_speed() -> f32 {
        0.1
    }

    fn valid_area() -> (Vec3, Vec3) {
        (Vec3::new(-1.5, -1.7, -0.4), Vec3::new(1.5, 1.6, 0.4))
    }

    fn valid_point_buffer() -> Vec3 {
        Vec3::new(0.2, 0.3, 0.1)
    }
}
