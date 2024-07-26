use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;

pub(super) const FISH_MAX: usize = 16;
pub(super) const FISH_AGING_SPEED_SECONDS: f32 = 1.;
pub(super) const FISH_SATIATION_MAX: u32 = 100;
pub(super) const FISH_LIFESPAN: u32 = 100;
pub(super) const FISH_BULK_MAX: u32 = 32;

#[derive(Component)]
pub struct Fish;

#[derive(Resource, Deref, DerefMut)]
pub struct FishRng(pub(super) ChaCha8Rng);
