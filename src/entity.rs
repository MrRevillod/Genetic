
use rand::Rng;
use std::ops::Add;
use colored::CustomColor;

use crate::utils;
use crate::position::*;
use crate::DIMENSIONS;

pub type Color<T> = (T, T, T);

/// Entity struct
/// 
/// # Attributes
/// 
/// * `id` - Entity identifier
/// * `values` - Entity values
/// * `killer` - Entity killer flag
/// * `alive` - Entity alive flag
/// * `position` - Entity position (Point)
/// 
/// # Methods
/// 
/// * `new` - Create a new Entity
/// * `is_killer` - Check if the entity is a killer
/// * `get_position` - Get the entity position
/// * `next_position` - Get the next entity position
/// 
/// # Examples

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub id: String,
    pub values: Vec<f64>,
    pub killer: bool,
    pub alive: bool,
    pub position: Position,
    pub color: CustomColor,
}


impl Entity {
  
    /// Create a new Entity
    /// 
    /// # Arguments
    /// 
    /// * `id` - Entity identifier
    /// * `position` - Entity position (Point)
    /// 
    /// # Returns
    /// 
    /// * `Entity` - New Entity with normalized 
    /// direction values and killer flag set randomly with a 10% chance

    pub fn new(id: String, position: Position) -> Self {

        let mut rng = rand::thread_rng();
        let mut values = vec![0.0; 8];
        
        for i in 0..8 {
            values[i] = rng.gen()
        }

        values = utils::normalize(&values);

        let color = utils::to_rgb((values[2], values[3], values[4]));

        let killer = rng.gen_bool(0.1);
        
        Entity { id, values, killer, position, alive: true, color }
    }

    pub fn from(id: String, values: Vec<f64>, killer: bool, position: Position, color: CustomColor) -> Self{
        Entity { id, values, killer, position, color, alive: true }
    }

    pub fn is_killer(&self) -> bool {
        self.killer
    }

    pub fn get_position(&self) -> Point {
        self.position.clone().unwrap()
    }

    /// Get the next entity position
    /// 
    /// # Arguments
    /// 
    /// * `dev_moves` - Vector of debug moves
    /// 
    /// # Returns
    /// 
    /// * `Point` - Next entity position (Point)

    pub fn next_position(&self, dev_moves: &mut Vec<String>) -> Point {
    
        let mut rng = rand::thread_rng();

        // Generate a random number between 0 and 1
        
        let prob = rng.gen::<f64>();
        let cumulatives = utils::cumulative(&self.values);

        // Find the index of the first value in the cumulative vector
        // that is greater than the random number
    
        let index = cumulatives.iter().position(|&v| v > prob).unwrap();

        // That index is the direction to move

        let dir = DIRECTIONS[index];

        // Get the current position of the entity
    
        let current_pos = self.get_position();

        // If the entity is in the last column, return the current position (no move)

        if current_pos.x == DIMENSIONS.1 as isize {
            return current_pos
        }

        // Calculate the next position and verify the limits
        // negatives and greater than the row limit
    
        let mut next_pos = current_pos + dir;

        if next_pos.x < 0 { next_pos.x = 0 }
        if next_pos.y < 0 { next_pos.y = 0 }
    
        if next_pos.y >= DIMENSIONS.0 as isize { next_pos.y = DIMENSIONS.0 as isize - 1 }

        dev_moves.push(format!("E-{} : {}", self.id, DEBUG_DIRECTIONS[&dir]));
    
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
