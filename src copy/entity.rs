
use rand::Rng;

use crate::utils;
use crate::position::*;
use crate::DIMENSIONS;

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
    pub id: u8,
    pub values: Vec<f64>,
    pub killer: bool,
    pub alive: bool,
    pub position: Position,
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

    pub fn new(id: u8, position: Position) -> Self {

        let mut rng = rand::thread_rng();
        let mut values = vec![0.0; 8];
        
        for i in 0..8 {
            values[i] = rng.gen()
        }

        values = utils::normalize(&values);
        
        let killer = rng.gen_bool(0.1);
        
        Entity { id, values, killer, position, alive: true }
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
    
        dev_moves.push(format!("E-{} : {}", self.id, DEBUG_DIRECTIONS[&dir]));

        // Get the current position of the entity
    
        let current_pos = self.get_position();

        // If the entity is in the last column, return the current position (no move)

        if current_pos.x >= DIMENSIONS.1 as isize {
            return current_pos
        }

        // Calculate the next position and verify the limits
        // negatives and greater than the row limit
    
        let mut next_pos = current_pos + dir;
   
        if next_pos.x < 0 { next_pos.x = 0 }
        if next_pos.y < 0 { next_pos.y = 0 }
    
        if next_pos.y >= DIMENSIONS.0 as isize { next_pos.y = DIMENSIONS.0 as isize - 1 }
    
        next_pos
    }

    // crossover -> mutate (muta al hacer la cruza)
}
    
