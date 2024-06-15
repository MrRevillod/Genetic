
use rand::Rng;
use colored::*;

use crate::position::*;
use crate::random::random;
use crate::entity::Entity;
use crate::utils::trunc_uuid;
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

#[derive(Clone, Debug)]
pub struct Poblation {
    pub entities: Vec<Entity>,
}

impl Poblation {

    pub fn new() -> Self {

        let mut entities: Vec<Entity> = Vec::new();

        let mut i = SAMPLE;

        while i > 0 {

            let random_row = random().gen_range(0..DIMENSIONS.0) as isize;
            let random_col = random().gen_range(0..=1) as isize;

            let new_pos = Point::new(random_col, random_row);

            if entities.iter().any(|e| e.get_position() == new_pos) {
                continue;
            }

            entities.push(Entity::new(Position::Some(new_pos)));

            i -= 1;
        }

        Poblation { entities }
    }

    pub fn run(&mut self) {

        self.show();

        for _ in 1..=N_ITERATIONS {
            
            let mut dead_entities = Vec::new();

            for i in 0..self.entities.len() {

                if !self.entities[i].alive { continue }
                let entity_next_pos = self.entities[i].next_position();

                let next_pos = self.entities.iter().position(
                    |e| e.get_position() == entity_next_pos && e.alive && e.id != self.entities[i].id
                );

                if let Some(j) = next_pos {

                    if self.entities[i].is_killer() && !self.entities[j].is_killer() {
                        dead_entities.push(j);
                        self.entities[i].position = Some(entity_next_pos);

                    } else if !self.entities[i].is_killer() && self.entities[j].is_killer() {
                        dead_entities.push(i);
                        self.entities[j].position = Some(entity_next_pos);

                    } else if self.entities[i].is_killer() && self.entities[j].is_killer() {
                        dead_entities.push(i); dead_entities.push(j);
                    }
                
                } else {
                    self.entities[i].position = Some(entity_next_pos);
                }
                
                for &i in dead_entities.iter() {
                    self.entities[i].alive = false;
                }

                self.entities[i].fitness += 1;
            }

            self.show();
        }
    }

    pub fn get_final_entities(&self) -> Vec<Entity> {

        let mut final_entities = Vec::new();

        for entity in self.entities.iter() {
            if entity.alive && entity.get_position().x == DIMENSIONS.1 as isize - 1 {
                final_entities.push(entity.clone());
            }
        }

        final_entities
    }

    pub fn show(&self) {

        let mut buffer = String::new();
        let total_width = DIMENSIONS.1 as usize * 7;
    
        buffer.push_str(&format!("\n+{:-<1$}+\n", "", total_width));
    
        for y in 0..DIMENSIONS.0 {

            for _ in 0..3 {

                buffer.push_str("|");
    
                for x in 0..DIMENSIONS.1 {
                    let current_post = Point::new(x as isize, y as isize);
    
                    if let Some(e) = self.entities.iter().find(|e| e.get_position() == current_post && e.alive) {
                        buffer.push_str(&format!("{}|", (0..6).map(|_| "*".custom_color(e.color).to_string()).collect::<String>()));
                    } else {
                        buffer.push_str(&format!(" {}|", " ".repeat(5)));
                    };
                }
    
                buffer.push_str("\n");
            }
    
            if y < DIMENSIONS.0 - 1 {
                buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));
            }
        }
    
        buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));
    
        std::thread::sleep(std::time::Duration::from_millis(150));
        std::process::Command::new("clear").status().unwrap();
    
        print!("{}", buffer);
    }
}

impl Poblation {

    pub fn run_debug(&mut self) {

        let killers = self.entities.iter()
            .filter(|e| e.is_killer()).map(|e| trunc_uuid(&e.id))
            .collect::<Vec<String>>()
        ;
        
        println!("\nInitial state");
        println!("Killers: {:?}", killers);

        for _ in 1..=N_ITERATIONS {
            
            let mut dead_entities = Vec::new();

            for i in 0..self.entities.len() {

                if !self.entities[i].alive { continue }
                
                let entity_next_pos = self.entities[i].next_position();
                println!("E{}: {:?}", trunc_uuid(&self.entities[i].id), entity_next_pos);

                let next_pos = self.entities.iter().position(
                    |e| e.get_position() == entity_next_pos && e.alive && e.id != self.entities[i].id
                );

                if let Some(j) = next_pos {

                    if self.entities[i].is_killer() && !self.entities[j].is_killer() {
                        dead_entities.push(j);
                        self.entities[i].position = Some(entity_next_pos);

                        // moves.push(format!("E{} killed E{}", trunc_uuid(&self.entities[i].id), trunc_uuid(&self.entities[j].id)));

                    } else if !self.entities[i].is_killer() && self.entities[j].is_killer() {
                        dead_entities.push(i);
                        self.entities[j].position = Some(entity_next_pos);
                        // moves.push(format!("E{} killed E{}", trunc_uuid(&self.entities[j].id), trunc_uuid(&self.entities[i].id)));

                    } else if self.entities[i].is_killer() && self.entities[j].is_killer() {
                        dead_entities.push(i); dead_entities.push(j);
                        // moves.push(format!("E{} and E{} killed each other", trunc_uuid(&self.entities[i].id), trunc_uuid(&self.entities[j].id)));
                    }
                
                } else {
                    self.entities[i].position = Some(entity_next_pos);
                }
                
                for &i in dead_entities.iter() {
                    self.entities[i].alive = false;
                }
            }

            // println!("Iteration {}", iteration);
            self.show_debug();
            // println!("Moves: {:?}\n", moves);

            // moves.clear();
        }
    }

    pub fn show_debug(&self) {
        println!();
        println!("+{:-<12}+", "-".repeat(DIMENSIONS.1 as usize * 12));

        for y in 0..DIMENSIONS.0 {
            print!("|");

            for x in 0..DIMENSIONS.1 {
                let current_post = Point::new(x as isize, y as isize);

                if let Some(entity) = self.entities.iter().find(|e| e.get_position() == current_post && e.alive) {
                    print!(" {:^10}|", format!("E{}", trunc_uuid(&entity.id)));

                } else {
                    print!(" {:^10}|", " ");
                }
            }

            println!();
            if y < DIMENSIONS.0 - 1 {
                println!("+{:-<12}+", "-".repeat(DIMENSIONS.1 as usize * 12));
            }
        }

        println!("+{:-<11}+", "-".repeat(DIMENSIONS.1 as usize * 12));
        println!();
    }
}
