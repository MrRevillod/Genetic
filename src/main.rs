
use entity::Entity;
use poblation::Poblation;
use position::Point;
use rand::{Rng, SeedableRng, StdRng};

pub mod utils;
pub mod entity;
pub mod position;
pub mod poblation;
pub mod random;

pub const SAMPLE: u8 = 15;
pub const DIMENSIONS: (u8, u8) = (10, 20);
pub const N_ITERATIONS: u8 = 50;
pub const MUTATION_PROB: f64 = 1.0;

fn main() {
    // let p = Poblation::new();

    let (E1, E2) = (Entity::new(String::from("E1"), Some(Point::new(0, 0))), Entity::new(String::from("E2"), Some(Point::new(0, 1))));

    dbg!(&E1);
    dbg!(&E2);

    let E3 = E1 + E2;

    dbg!(&E3);
}
