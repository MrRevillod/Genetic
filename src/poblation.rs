
use std::thread::sleep;

use colored::*;
use rand::Rng;
use colored::*;

use crate::position::*;
use crate::entity::Entity;
use crate::{DIMENSIONS, SAMPLE, N_ITERATIONS};

/// Poblation struct
/// 
/// # Attributes
/// 
/// * `entities` - Poblation entities
/// 
/// # Methods
/// 
/// * `new` - Create a new Poblation
/// * `run` - Run the simulation of the Poblation
/// * `show` - Show the Poblation state

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
                entities.len().to_string(), Position::Some(new_pos)
            ));

            i -= 1;
        }

        Poblation { entities }
    }

    pub fn run(&mut self) {

        let mut dev_moves: Vec<String> = Vec::new();
        let killers: Vec<u8> = self.entities.iter().filter(|e| e.is_killer()).map(|e| e.id).collect();
        let mut kills_moves: Vec<String> = Vec::new();
        
        // println!("\nInitial state");
        // println!("Killers: {:?}", killers);
        self.show_2();

        for iteration in 1..=N_ITERATIONS {
            
            let mut dead_entities = Vec::new();

            for i in 0..self.entities.len() {

                if self.entities[i].alive == false {
                    continue;
                }
                
                let entity_next_pos = self.entities[i].next_position(&mut dev_moves);

                let next_pos = self.entities.iter().position(
                    |e| e.get_position() == entity_next_pos && e.alive && e.id != self.entities[i].id
                );

                if let Some(j) = next_pos {

                    if self.entities[i].is_killer() && !self.entities[j].is_killer() {
                        dead_entities.push(j);
                        kills_moves.push(format!("E{} killed E{}", self.entities[i].id, self.entities[j].id));
                    } else if !self.entities[i].is_killer() && self.entities[j].is_killer() {
                        dead_entities.push(i);
                        kills_moves.push(format!("E{} killed E{}", self.entities[j].id, self.entities[i].id));
                    } else if self.entities[i].is_killer() && self.entities[j].is_killer() {
                        dead_entities.push(i);
                        dead_entities.push(j);
                        kills_moves.push(format!("E{} and E{} killed each other", self.entities[i].id, self.entities[j].id));
                    }
                
                } else {
                    self.entities[i].position = Some(entity_next_pos);
                }
                
                for &i in dead_entities.iter() {
                    self.entities[i].alive = false;
                }
            }

            // println!("Iteration {}", iteration);
            self.show_2();
            // println!("Moves: {:?}\n", dev_moves);
            // println!("Kills: {:?}", kills_moves);

            dev_moves.clear();
            kills_moves.clear();
        }
    }
    
    pub fn show(&self) {
        println!();
        println!("+{:-<5}+", "-".repeat(DIMENSIONS.1 as usize * 6));

        for y in 0..DIMENSIONS.0 {
            print!("|");

            for x in 0..DIMENSIONS.1 {
                let current_post = Point::new(x as isize, y as isize);

                if let Some(entity) = self.entities.iter().find(|e| e.get_position() == current_post) {
                    print!("E{} ", entity.id.custom_color(entity.color));
                } else {
                    print!("{:^5} |", " ");
                }
            }

            println!();
            if y < DIMENSIONS.0 - 1 {
                println!("+{:-<5}+", "-".repeat(DIMENSIONS.1 as usize * 6));
            }
        }

        println!("+{:-<5}+", "-".repeat(DIMENSIONS.1 as usize * 6));
        println!();
    }


pub fn show_2(&self) {

    std::thread::sleep(std::time::Duration::from_millis(500));
    std::process::Command::new("clear").status().unwrap();

    println!();
    println!("+{:-<11}+", "-".repeat(DIMENSIONS.1 as usize * 12));

    for y in 0..DIMENSIONS.0 {
        for _ in 0..3 {
            print!("| ");

            for x in 0..DIMENSIONS.1 {
                let current_post = Point::new(x as isize, y as isize);

                if let Some(entity) = self.entities.iter().find(|e| e.get_position() == current_post && e.alive) {
                    print!("{:^11} |", (0..9).map(|_| "*".green().to_string()).collect::<String>());
                } else {
                    print!("{:^11} |", " ".repeat(8));
                }
            }

            println!();
        }
        if y < DIMENSIONS.0 - 1 {
            println!("+{:-<11}+", "-".repeat(DIMENSIONS.1 as usize * 12));
        }
    }

    println!("+{:-<11}+", "-".repeat(DIMENSIONS.1 as usize * 12));
    println!();
}
}
