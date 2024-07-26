use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;

pub(super) const FISH_MAX: usize = 12;
pub(super) const FISH_SPAWN_INTERVAL_SECONDS: f32 = 5.;
pub(super) const FISH_AGING_INTERVAL_SECONDS: f32 = 10.;
pub(super) const FISH_SATIATION_MAX: u32 = 64;
pub(super) const FISH_AVERAGE_LONGEVITY: u32 = 64;
pub(super) const FISH_BULK_MAX: u32 = 32;

#[derive(Component)]
pub struct Fish;

#[derive(Resource, Deref, DerefMut)]
pub struct FishRng(pub(super) ChaCha8Rng);
