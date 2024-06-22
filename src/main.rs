
pub mod utils;
pub mod entity;
pub mod random;
pub mod position;
pub mod poblation;

use poblation::Poblation;

pub mod constants {
    pub const SAMPLE: usize = 16;
    pub const N_ITERATIONS: usize = 100;
    pub const SEED: [u8; 32] = [10; 32];
    pub const DIMENSIONS: (usize, usize) = (12, 20);
    pub const N_GENERATIONS: usize = 500;
    pub const SHOW_THRESHOLD: usize = 10;
    pub const MUTATION_PROBABILTY: f64 = 0.1;
    pub const KILLER_PROBABILITY: f64 = 0.05;
    
    pub const P: f64 = 0.5;
}

// use textplots::*;

fn main() {

    Poblation::new().run();

    // let mut points = Vec::new();
    
    // for x in -10..=1000 {
    //     let x_f32 = x as f32;
    //     points.push((x_f32, x_f32.exp()));
    // }
    
    // Chart::default().lineplot(&Shape::Lines(&points)).display();
}
