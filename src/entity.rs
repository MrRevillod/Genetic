
use rand::Rng;
use uuid::Uuid;
use std::ops::Add;
use colored::CustomColor;

use crate::utils;
use crate::DIMENSIONS;
use crate::position::*;
use crate::utils::random;
use crate::MUTATE_PROBABILTY;

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
    pub id: Uuid,
    pub values: Vec<f64>,
    pub killer: bool,
    pub alive: bool,
    pub position: Position,
    pub color: CustomColor,
    pub fitness: usize,
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

    pub fn new(position: Position) -> Self {

        let mut values = vec![0.0; 8];
        
        for i in 0..8 {
            values[i] = utils::random().gen::<f64>()
        }

        utils::normalize(&mut values);

        let color = utils::to_rgb((values[2], values[3], values[4]));
        let killer = utils::random().gen_bool(0.05);
        
        Entity { id: utils::uuid(), values, killer, position, alive: true, color, fitness: 0 }
    }

    /// Create a new Entity from a given values
    /// 
    /// # Arguments
    /// 
    /// * `values` - Entity values
    /// * `killer` - Entity killer flag
    /// * `position` - Entity position (Point)
    /// * `color` - Entity color

    pub fn from(values: Vec<f64>, killer: bool, 
        position: Position, color: CustomColor) -> Self {

        Entity { 
            id: utils::uuid(), 
            values, 
            killer, 
            position, 
            color, 
            alive: true,
            fitness: 0,
        }
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

    pub fn next_position(&mut self) -> Point {
    
        // Generate a random number between 0 and 1
        
        let prob = utils::random().gen::<f64>();
        let cumulatives = utils::cumulative(&self.values);

        // Find the index of the first value in the cumulative vector
        // that is greater than the random number
    
        let index = cumulatives.iter().position(|&v| v > prob).unwrap();

        // That index is the direction to move

        let dir = DIRECTIONS[index];

        // Get the current position of the entity
    
        let current_pos = self.get_position();

        if current_pos.x == (DIMENSIONS.1 - 1) as isize {
            return current_pos;
        }

        // Calculate the next position and verify the limits
        // negatives and greater than the row limit
    
        let next_pos = current_pos + dir;

        // Check if next_pos is outside the limits
        if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= DIMENSIONS.1 as isize || next_pos.y >= DIMENSIONS.0 as isize {
            return current_pos
        }

        self.fitness += 1;
        next_pos
    }

    pub fn mutate(&mut self) {
        
        let index = utils::random().gen_range(0..9);

        if index == 8 {
            self.killer = !self.killer;
            return
        }

        self.values[index] = utils::random().gen::<f64>();
        utils::normalize(&mut self.values);
    }
}
    
impl Add for Entity {

    type Output = (Entity, Entity);

    fn add(self, rhs: Self) -> Self::Output {

        let c1_1 = self.values[0..=3].to_vec();
        let c1_2 = self.values[4..=7].to_vec();
        
        let c2_1 = rhs.values[0..=3].to_vec();
        let c2_2 = rhs.values[4..=7].to_vec();

        let children_1_v = [c1_1, c2_2].concat();
        let children_2_v = [c2_1, c1_2].concat();

        let children_1_color = utils::to_rgb((children_1_v[2], children_1_v[3], children_1_v[4]));
        let children_2_color = utils::to_rgb((children_2_v[2], children_2_v[3], children_2_v[4]));

        let mut children_1 = Entity::from(children_1_v, rhs.killer, Position::None, children_1_color);
        let mut children_2 = Entity::from(children_2_v, self.killer, Position::None, children_2_color);

        if random().gen::<f64>() <= MUTATE_PROBABILTY {

            let mutation_target = random().gen_range(0..=2);

            match mutation_target {
                0 => children_1.mutate(),
                1 => children_2.mutate(),

                _ => { children_1.mutate(); children_2.mutate() }
            }
        }

        (children_1, children_2)
    }
}
