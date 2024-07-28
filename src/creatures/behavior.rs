use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::pellets::Pellet;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CreatureRng>();
}

pub enum CreatureBehaviorVariant {
    Debut,
    Idle,
    SwimRight,
    SwimLeft,
    SeekPoint(Vec3),
    SeekPellet(Entity),
}

#[derive(Component)]
pub struct CreatureBehavior {
    pub(super) timer: Timer,
    pub(super) variant: CreatureBehaviorVariant,
}

impl Default for CreatureBehavior {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
            variant: CreatureBehaviorVariant::Debut,
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct CreatureRng(pub ChaCha8Rng);

impl Default for CreatureRng {
    fn default() -> Self {
        Self(ChaCha8Rng::seed_from_u64(19878367467712))
    }
}

pub trait CreatureOperations {
    fn do_behavior(
        &mut self,
        commands: &mut Commands,
        rng: &mut CreatureRng,
        time: &Time,
        pellets: &Query<(Entity, &mut Transform), (With<Pellet>, Without<CreatureBehavior>)>,
    ) {
        match self.behavior().variant {
            CreatureBehaviorVariant::Debut => self.behavior_debut(time, rng),
            CreatureBehaviorVariant::Idle => self.behavior_idle(time),
            CreatureBehaviorVariant::SwimRight => self.behavior_swim_right(time),
            CreatureBehaviorVariant::SwimLeft => self.behavior_swim_left(time),
            CreatureBehaviorVariant::SeekPoint(point) => self.behavior_seek_point(time, point),
            CreatureBehaviorVariant::SeekPellet(pellet_id) => {
                self.behavior_seek_pellet(time, rng, pellets.get(pellet_id), commands)
            }
        }

        self.clamp();
    }

    fn decide_behavior(&mut self, time: &Time) {
        self.behavior().timer.tick(time.delta());

        if self.behavior().timer.just_finished() {
            match self.behavior().variant {
                CreatureBehaviorVariant::Idle => {
                    if self.transform().rotation.y == 0. {
                        self.start_swim_left();
                    } else {
                        self.start_swim_right();
                    }
                }
                CreatureBehaviorVariant::SwimRight => self.start_idle(),
                CreatureBehaviorVariant::SwimLeft => self.start_idle(),
                _ => {}
            }
        }
    }

    fn detect_pellet(
        &mut self,
        pellets: &Query<(Entity, &Transform), (With<Pellet>, Without<CreatureBehavior>)>,
    ) {
        let (best_pellet_id, best_rank) = pellets.iter().fold(
            (None, f32::MAX),
            |(best_pellet_id, best_rank), (pellet_id, pellet_transform)| {
                let rank = self.rank_pellet(pellet_transform);
                if rank < best_rank {
                    (Some(pellet_id), rank)
                } else {
                    (best_pellet_id, best_rank)
                }
            },
        );

        if let Some(best_pellet_id) = best_pellet_id {
            if self.check_pellet(best_rank) {
                self.start_seek_pellet(best_pellet_id);
            }
        }
    }

    fn start_idle(&mut self) {
        self.behavior().variant = CreatureBehaviorVariant::Idle;
        self.behavior().timer.reset();
    }

    fn start_swim_right(&mut self) {
        self.face_right();
        self.behavior().variant = CreatureBehaviorVariant::SwimRight;
        self.behavior().timer.reset();
    }

    fn start_swim_left(&mut self) {
        self.face_left();
        self.behavior().variant = CreatureBehaviorVariant::SwimLeft;
        self.behavior().timer.reset();
    }

    fn start_seek_point(&mut self, rng: &mut CreatureRng) {
        let point = Self::valid_random_point(rng);

        if self.transform().translation.x < point.x {
            self.face_right();
        } else {
            self.face_left();
        }

        self.behavior().variant = CreatureBehaviorVariant::SeekPoint(point);
        self.behavior().timer.reset();
    }

    fn start_seek_pellet(&mut self, pellet_id: Entity) {
        self.behavior().variant = CreatureBehaviorVariant::SeekPellet(pellet_id);
        self.behavior().timer.reset();
    }

    fn behavior_debut(&mut self, time: &Time, rng: &mut CreatureRng);
    fn behavior_idle(&mut self, time: &Time);

    fn behavior_swim_right(&mut self, time: &Time) {
        self.transform().translation.x += time.delta_seconds() * Self::base_speed();

        let (_, max) = Self::valid_area();
        if self.transform().translation.x > max.x {
            self.start_swim_left();
        }
    }

    fn behavior_swim_left(&mut self, time: &Time) {
        self.transform().translation.x -= time.delta_seconds() * Self::base_speed();

        let (min, _) = Self::valid_area();
        if self.transform().translation.x < min.x {
            self.start_swim_right();
        }
    }

    fn behavior_seek_point(&mut self, time: &Time, target: Vec3) {
        self.transform().translation = self
            .transform()
            .translation
            .move_towards(target, time.delta_seconds() * Self::base_speed());

        if self.transform().translation.distance(target) < 0.1 {
            self.start_idle();
        }
    }

    fn behavior_seek_pellet(
        &mut self,
        time: &Time,
        rng: &mut CreatureRng,
        pellet: Result<(Entity, &Transform), QueryEntityError>,
        commands: &mut Commands,
    );

    fn face_right(&mut self);
    fn face_left(&mut self);

    fn clamp(&mut self) {
        let (min, max) = Self::valid_area();
        self.transform().translation = self.transform().translation.clamp(min, max);
    }

    fn rank_pellet(&mut self, pellet_transform: &Transform) -> f32;
    fn check_pellet(&mut self, rank: f32) -> bool;

    fn behavior(&mut self) -> &mut CreatureBehavior;
    fn transform(&mut self) -> &mut Transform;

    fn base_speed() -> f32;
    fn valid_area() -> (Vec3, Vec3);

    fn valid_random_point(rng: &mut CreatureRng) -> Vec3 {
        let (min, max) = Self::valid_area();
        Cuboid::from_corners(min + Self::valid_point_buffer(), max - Self::valid_point_buffer()).sample_interior(&mut rng.0)
    }

    fn valid_point_buffer() -> Vec3 {
        Vec3::splat(0.0)
    }
}

