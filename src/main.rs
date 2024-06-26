
pub mod utils;
pub mod entity;
pub mod random;
pub mod position;
pub mod poblation;

use poblation::Poblation;
use utils::continue_prompt;

pub mod constants {

    pub const SAMPLE: usize = 16;
    pub const N_ITERATIONS: usize = 100;
    pub const DIMENSIONS: (usize, usize) = (12, 20);
    pub const N_GENERATIONS: usize = 500;
    pub const SHOW_THRESHOLD: usize = 10;
    pub const SEED: [u8; 32] = [10; 32];
    pub const MUTATION_PROBABILTY: f64 = 0.05;
    pub const KILLER_PROBABILITY: f64 = 0.2;
    pub const P: f64 = 0.5;
}

fn main() {

    let mut poblation = Poblation::new();

    poblation.run();
    continue_prompt();
    
    poblation.graphic("murders");
    continue_prompt();
    
    poblation.graphic("winners");
}
