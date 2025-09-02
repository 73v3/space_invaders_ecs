// src/random.rs

use bevy::prelude::*;
// Import WyRand and the necessary query components from bevy_rand
use bevy_rand::prelude::{EntropyPlugin, GlobalEntropy, WyRand};
// Import the Rng trait to use methods like .gen()
use rand_core::RngCore;
use std::time::{SystemTime, UNIX_EPOCH};

/// Plugin for handling random number generation with WyRand
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;

        // CORRECTED: The `with_seed` function now expects a byte array.
        // We convert the u64 seed to a little-endian byte array.
        app.add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_le_bytes()));
    }
}

pub fn random_float(rng: &mut GlobalEntropy<WyRand>) -> f32 {
    (rng.next_u32() as f32) / (u32::MAX as f32)
}
