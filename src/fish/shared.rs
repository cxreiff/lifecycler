use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;

pub(super) const FISH_MAX: usize = 5;

#[derive(Component)]
pub struct Fish;

#[derive(Resource, Deref, DerefMut)]
pub struct FishRng(pub(super) ChaCha8Rng);
