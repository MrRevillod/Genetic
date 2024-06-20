
pub mod utils;
pub mod entity;
pub mod random;
pub mod position;
pub mod poblation;

use poblation::Poblation;
use textplots::{Chart, Plot, Shape};

pub const SAMPLE: u8 = 16;
pub const N_ITERATIONS: u8 = 100;
pub const SEED: [u8; 32] = [10; 32];
pub const DIMENSIONS: (u8, u8) = (12, 20);
pub const N_GENERATIONS: usize = 100;
pub const SHOW_THRESHOLD: usize = 10;
pub const MUTATION_PROBABILTY: f64 = 0.1;

pub const P: f64 = 0.5;

// use textplots::*;

fn main() {

    let mut poblation: Poblation = Poblation::new();
    poblation.run();

    let data: Vec<(f32, f32)> = poblation.get_data();
    Chart::new(180, 60, 0.0, N_GENERATIONS as f32)
        .lineplot(&Shape::Lines(&data))
        .display();

    // let mut points = Vec::new();
    
    // for x in -10..=1000 {
    //     let x_f32 = x as f32;
    //     points.push((x_f32, x_f32.exp()));
    // }
    
    // Chart::default().lineplot(&Shape::Lines(&points)).display();
}
