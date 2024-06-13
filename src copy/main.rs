
use poblation::Poblation;

pub mod utils;
pub mod entity;
pub mod position;
pub mod poblation;

pub const SAMPLE: u8 = 7;
pub const DIMENSIONS: (u8, u8) = (5, 10);
pub const CELL_SIZE: f32 = 40.0;
pub const N_ITERATIONS: u8 = 50;

fn main() {

    Poblation::new().run();
}
