
use std::env::args;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, SeedableRng};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::SEED;

pub struct RngGenerator;
pub type RandomGenerator = Arc<Mutex<StdRng>>;

impl RngGenerator {

    pub fn new(rng: StdRng) -> RandomGenerator {
        Arc::new(Mutex::new(rng))
    }
}

lazy_static!(

    pub static ref RNG: RandomGenerator = {

        let args = args().collect::<Vec<String>>();

        if args[1] == "ws" {
            RngGenerator::new(SeedableRng::from_seed(SEED))
        }

        else if args[1] == "ns" { 
            RngGenerator::new(StdRng::from_entropy())
        }

        else {
            panic!("Invalid argument")
        }
    };
);

pub fn random() -> MutexGuard<'static, StdRng> {
    return RNG.lock().unwrap()
}
