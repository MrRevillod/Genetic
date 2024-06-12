
use rand::Rng;

use crate::position::*;
use crate::entity::Entity;
use crate::{DIMENSIONS, SAMPLE};

#[derive(Clone, Debug, PartialEq)]
pub struct Poblation {
    pub entities: Vec<Entity>
}

impl Poblation {

    pub fn new() -> Self {

        let mut entities: Vec<Entity> = Vec::new();

        let mut i = SAMPLE;

        while i > 0 {

            let mut rng = rand::thread_rng();

            let random_row = rng.gen_range(0..DIMENSIONS.0) as isize;
            let random_col = rng.gen_range(0..=1) as isize;

            let new_pos = Point::new(random_col, random_row);

            if entities.iter().any(|e| e.get_position() == new_pos) {
                continue;
            }

            entities.push(Entity::new(
                entities.len() as u8, Position::Some(new_pos)
            ));

            i -= 1;
        }

        Poblation { entities }
    }
    
    pub fn show(&self) {
    
        for y in 0..DIMENSIONS.0 {

            for x in 0..DIMENSIONS.1 {

                let current_post = Point::new(x as isize, y as isize);

                if let Some(entity) = self.entities.iter().find(|e| e.get_position() == current_post) {
                    print!("E{} ", entity.id);
                } else {
                    print!("nn ");
                }
            }

            println!()

        }
    }
}

// }

//     pub fn run(&mut self) {

//         self.show();
    
//         for i in 0..N_ITERATIONS {
            
//             for i in 0..self.poblation.len() {
                
//                 let next_pos = self.poblation[i].next_position(&mut self.moves);
    
//                 if !self.poblation.iter().any(|e| e.get_position() == next_pos) {
//                     self.poblation[i].position = PointRef::new(Some(next_pos));
//                     continue
//                 }
                
//                 // logica de competencia
//             }
            
//             println!("Iteration {}", i);
//             self.show();
//             println!("\nMoves: {:?}\n", self.moves);
//             self.moves.clear();
//         }
//     }
// }
