
pub mod utils;
pub mod entity;
pub mod random;
pub mod position;
pub mod poblation;

use poblation::Poblation;

pub const SAMPLE: u8 = 16;
pub const N_ITERATIONS: u8 = 100;
pub const SEED: [u8; 32] = [10; 32];
pub const DIMENSIONS: (u8, u8) = (12, 20);
pub const N_GENERATIONS: usize = 1;

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
