
use colored::CustomColor;
use rand::Rng;
use std::ops::Add;

use crate::utils;
use crate::position::*;
use crate::DIMENSIONS;

pub type Color<T> = (T, T, T);

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub id: String,
    pub values: Vec<f64>,
    pub killer: bool,
    pub position: Position,
    pub color: CustomColor,
}


impl Entity {

    pub fn new(id: String, position: Position) -> Self {

        let mut rng = rand::thread_rng();
        let mut values = vec![0.0; 8];
        
        for i in 0..8 {
            values[i] = rng.gen()
        }

        values = utils::normalize(&values);

        let color = utils::to_rgb((values[2], values[3], values[4]));

        let killer = rng.gen_bool(0.1);
        
        Entity { id, values, killer, position , color}
    }

    pub fn from(id: String, values: Vec<f64>, killer: bool, position: Position, color: CustomColor) -> Self{

        Entity{id, values, killer, position, color}
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
    
impl Add for Entity {

    type Output = (Entity, Entity);

    fn add(self, rhs: Self) -> Self::Output {

        let father_c1 = self.values[0..=3].to_vec();
        let mother_c1 = rhs.values[4..=7].to_vec();

        let father_c2 = self.values[4..=7].to_vec();
        let mother_c2 = rhs.values[0..=3].to_vec();

        let c1_values = [father_c1, mother_c1].concat();
        let c2_values = [father_c2, mother_c2].concat();

        let c1_id = format!("c_{}_{}", self.id, rhs.id);
        let c2_id = format!("c_{}_{}", rhs.id, self.id);

        let c1_killer = rhs.killer;
        let c2_killer = self.killer;

        let c1_color = utils::to_rgb((c1_values[2], c1_values[3], c1_values[4]));
        let c2_color = utils::to_rgb((c2_values[2], c2_values[3], c2_values[4]));

        (
            Entity::from(c1_id, c1_values, c1_killer, Position::None, c1_color),
            Entity::from(c2_id, c2_values, c2_killer, Position::None, c2_color)
        )
    }
}
