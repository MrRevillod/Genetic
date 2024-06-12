
use rand::Rng;

use crate::utils;
use crate::position::*;
use crate::DIMENSIONS;

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub id: u8,
    pub values: Vec<f64>,
    pub killer: bool,
    pub position: Position,
}

impl Entity {

    pub fn new(id: u8, position: Position) -> Self {

        let mut rng = rand::thread_rng();
        let mut values = vec![0.0; 8];
        
        for i in 0..8 {
            values[i] = rng.gen()
        }

        values = utils::normalize(&values);
        
        let killer = rng.gen_bool(0.1);
        
        Entity { id, values, killer, position }
    }
    
    pub fn is_killer(&self) -> bool {
        self.killer
    }

    pub fn get_position(&self) -> Point {
        self.position.clone().unwrap()
    }

    pub fn next_position(&mut self, dev_moves: &mut Vec<String>) -> Point {
    
        let mut rng = rand::thread_rng();
        
        let prob = rng.gen::<f64>();
        let cumulatives = utils::cumulative(&self.values);
    
        let index = cumulatives.iter().position(|&v| v > prob).unwrap();

        let dir = DIRECTIONS[index];
    
        dev_moves.push(format!("E-{} : {}", self.id, DEBUG_DIRECTIONS[&dir]));
    
        let current_pos = self.get_position();
    
        let mut next_pos = current_pos + dir;
   
        if next_pos.x < 0 { next_pos.x = 0 }
        if next_pos.y < 0 { next_pos.y = 0 }
    
        if next_pos.x >= DIMENSIONS.1 as isize { next_pos.x = DIMENSIONS.1 as isize - 1 }
        if next_pos.y >= DIMENSIONS.0 as isize { next_pos.y = DIMENSIONS.0 as isize - 1 }
    
        next_pos
    }

    // crossover -> mutate (muta al hacer la cruza)
}
    
