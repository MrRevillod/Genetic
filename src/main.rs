
use poblation::Poblation;

pub mod utils;
pub mod entity;
pub mod position;
pub mod poblation;

pub const SAMPLE: u8 = 15;
pub const DIMENSIONS: (u8, u8) = (10, 20);
pub const CELL_SIZE: f32 = 40.0;

fn main() {

    let p = Poblation::new();

    p.show();
    // for x in 0..2 {
    //     dbg!(&p.entities[x]);
    // }

    // let cross = p.entities[0].clone() + p.entities[1].clone();
    // dbg!(cross);
}
