
use lazy_static::lazy_static;

use rand::prelude::*;
use rand::rng::StdRng;
use std::sync::Mutex;

lazy_static! {
    pub static ref RNG: Mutex<StdRng> = {
        let seed: u8 = 5;
        let seed_array: [u8, 32] = [seed; 32];
        let rng = SeedableRng::from_seed(seed_array);
        Mutex::new(rng)
    };
}