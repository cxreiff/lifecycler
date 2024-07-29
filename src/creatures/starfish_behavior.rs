use bevy::prelude::*;

use super::behavior::{CreatureBehavior, CreatureBehaviorVariant, CreatureOperations, CreatureRng};

pub struct StarfishOperations<'a> {
    transform: &'a mut Transform,
    behavior: &'a mut CreatureBehavior,
}

impl<'a> StarfishOperations<'a> {
    pub(super) fn new(transform: &'a mut Transform, behavior: &'a mut CreatureBehavior) -> Self {
        Self {
            transform,
            behavior,
        }
    }
}

impl<'a> CreatureOperations for StarfishOperations<'a> {
    fn decide_behavior(&mut self, time: &Time, rng: &mut CreatureRng) {
        self.behavior().timer.tick(time.delta());

        if self.behavior().timer.just_finished() {
            match self.behavior().variant {
                CreatureBehaviorVariant::Idle => self.start_seek_point(rng),
                _ => self.start_idle(),
            }
        }
    }

    fn behavior_debut(&mut self, _time: &Time, _rng: &mut CreatureRng) {
        self.start_idle()
    }

    fn behavior_idle(&mut self, time: &Time) {
        self.transform()
            .rotate_z((time.elapsed_seconds() / 10.).sin() * Self::base_speed() * -0.1);
    }

    fn base_speed() -> f32 {
        0.01
    }

    fn valid_area() -> (Vec3, Vec3) {
        (Vec3::new(-1.4, -1.4, -0.41), Vec3::new(1.4, 1.4, -0.41))
    }

    fn behavior(&mut self) -> &mut CreatureBehavior {
        self.behavior
    }

    fn transform(&mut self) -> &mut Transform {
        self.transform
    }

    fn valid_point_buffer() -> Vec3 {
        Vec3::new(0.1, 0.1, 0.)
    }
}
