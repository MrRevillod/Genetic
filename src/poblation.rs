
use std::time::Duration;
use std::vec;

use rand::Rng;
use colored::*;

use crate::{position::*, N_GENERATIONS, P, SHOW_THRESHOLD};
use crate::random::random;
use crate::entity::Entity;
use crate::utils::{cumulative, normalize, trunc_uuid};
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

    pub fn assign_positions(&self, entities: &mut Vec<Entity>) {

        let mut i = 0;

        while i < entities.len() {

            let random_row = random().gen_range(0..DIMENSIONS.0) as isize;
            let random_col = random().gen_range(0..=1) as isize;
            
            let new_pos = Point::new(random_col, random_row);

            if entities.iter().any(|e| e.position.is_some() && e.get_position() == new_pos) {
                continue;
            }

            entities[i].position = Position::Some(new_pos);

            i += 1;
        }
    }

    pub fn selection(&self, final_entities: Vec<Entity>) -> Vec<Entity> {
        
        let mut entities = final_entities;
        let mut new_entities: Vec<Entity> = Vec::new();

        while entities.len() != DIMENSIONS.0 as usize {
            entities.push(Entity::new(Position::None));
        }

        let mut probs: Vec<f64> = vec![P];

        for i in 1..DIMENSIONS.0 as usize {
            probs.push(P*(1 as f64 - P).powi(i as i32));
        }

        probs = normalize(&probs);

        let cumulative = cumulative(&probs);

        while new_entities.len() != SAMPLE as usize {
            
            let prob_1 = random().gen::<f64>();
            let mut prob_2 = random().gen::<f64>();

            while prob_1 == prob_2 {
                prob_2 = random().gen::<f64>();
            }

            let c1_index = cumulative.iter().position(|&p| p > prob_1).unwrap();
            let mut c2_index = cumulative.iter().position(|&p| p > prob_2).unwrap();

            while c1_index == c2_index {
                prob_2 = random().gen::<f64>();
                c2_index = cumulative.iter().position(|&p| p > prob_2).unwrap();
            }

            let childrens = entities[c1_index].clone() + entities[c2_index].clone();

            new_entities.push(childrens.0);
            new_entities.push(childrens.1);
        }

        self.assign_positions(&mut new_entities);

        new_entities
    }

    pub fn run(&mut self) {

        let mut generation = 1;

        while generation <= N_GENERATIONS {

            // println!("generation: {}", generation);

            let mut finished_entities: Vec<Entity> = Vec::new();
    
            // self.show_debug();
    
            for iteration in 1..=N_ITERATIONS {
                
                let mut dead_entities: Vec<usize> = Vec::new();
    
                for i in 0..self.entities.len() {
    
                    if !self.entities[i].alive { continue }
    
                    if finished_entities.iter().any(|e| e.id == self.entities[i].id) { 
                        continue 
                    }
    
                    if self.entities[i].get_position().x == DIMENSIONS.1 as isize - 1 {
                        finished_entities.push(self.entities[i].clone());
                        continue;
                    }
    
                    let entity_next_pos = self.entities[i].next_position();
    
                    let next_pos = self.entities.iter().position(
                        |e| e.get_position() == entity_next_pos && e.alive && e.id != self.entities[i].id
                    );
    
                    if let Some(j) = next_pos {
    
                        if self.entities[i].is_killer() && !self.entities[j].is_killer() {
                            dead_entities.push(j);
                            self.entities[j].position = Position::Some(entity_next_pos);
    
                        } else if !self.entities[i].is_killer() && self.entities[j].is_killer() {
                            dead_entities.push(i);
                            self.entities[j].position = Position::Some(entity_next_pos);
    
                        } else if self.entities[i].is_killer() && self.entities[j].is_killer() {
                            dead_entities.push(i); dead_entities.push(j);
                        }
                    
                    } else {
                        self.entities[i].position = Some(entity_next_pos);
                    }
                    
                    for &i in dead_entities.iter() {
                        self.entities[i].alive = false
                    }
                }

                if generation % SHOW_THRESHOLD == 0 {
                    self.show(generation, iteration as usize)
                }
            }

            finished_entities.sort_by(|a, b| a.fitness.cmp(&b.fitness));

            if finished_entities.len() == DIMENSIONS.0 as usize {
                self.show(generation, N_ITERATIONS as usize);
                break
            }

            self.entities = self.selection(finished_entities);

            generation += 1;
        }

        return
    }

    pub fn show(&self, generation_arg: usize, iteration_arg: usize) {
        
        let mut buffer = String::new();
        let total_width = DIMENSIONS.1 as usize * 7;

        buffer.push_str(&format!("\x1B[2J\x1B[1;1H"));
        buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));

        let header = format!("| Generation: {:<5} Movement: {:<5} {}|", generation_arg, iteration_arg, " ".repeat(total_width - 36));
        buffer.push_str(&format!("{}\n", header));
        buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));

        for y in 0..DIMENSIONS.0 {
            for _ in 0..3 {
                buffer.push_str("|");

                for x in 0..DIMENSIONS.1 {
                    let current_pos = Point::new(x as isize, y as isize);

                    if let Some(e) = self.entities.iter().find(|e| e.get_position() == current_pos && e.alive) {
                        if e.is_killer() {
                            buffer.push_str(&format!("{}", (0..2).map(|_| "*".custom_color(e.color).to_string()).collect::<String>()));
                            buffer.push_str(&format!("{}", (0..2).map(|_| "*".white().to_string()).collect::<String>()));
                            buffer.push_str(&format!("{}|", (0..2).map(|_| "*".custom_color(e.color).to_string()).collect::<String>()));
                        } else {
                            buffer.push_str(&format!("{}|", (0..6).map(|_| "*".custom_color(e.color).to_string()).collect::<String>()));
                        }
                    } else {
                        buffer.push_str(&format!(" {}|", " ".repeat(5)));
                    }
                }

                buffer.push_str("\n");
            }

            if y < DIMENSIONS.0 - 1 {
                buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));
            }
        }

        buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));

        print!("{}", buffer);

        std::thread::sleep(Duration::from_millis(15));
    }

}

//     pub fn show(&self) {

//         let mut buffer = String::new();
//         let total_width = DIMENSIONS.1 as usize * 7;
    
//         buffer.push_str(&format!("\x1B[2J\x1B[1;1H"));
//         buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));
    
//         for y in 0..DIMENSIONS.0 {

//             for _ in 0..3 {

//                 buffer.push_str("|");
    
//                 for x in 0..DIMENSIONS.1 {

//                     let current_pos = Point::new(x as isize, y as isize);
    
//                     if let Some(e) = self.entities.iter().find(|e| e.get_position() == current_pos && e.alive) {

//                         if e.is_killer() {
//                             buffer.push_str(&format!("{}", (0..2).map(|_| "*".custom_color(e.color).to_string()).collect::<String>()));
//                             buffer.push_str(&format!("{}", (0..2).map(|_| "*".white().to_string()).collect::<String>()));
//                             buffer.push_str(&format!("{}|", (0..2).map(|_| "*".custom_color(e.color).to_string()).collect::<String>()));
//                         } else {
//                             buffer.push_str(&format!("{}|", (0..6).map(|_| "*".custom_color(e.color).to_string()).collect::<String>()));
//                         }

//                     } else {
//                         buffer.push_str(&format!(" {}|", " ".repeat(5)));
//                     }
//                 }
    
//                 buffer.push_str("\n");
//             }
    
//             if y < DIMENSIONS.0 - 1 {
//                 buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));
//             }
//         }
    
//         buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));
    
//         print!("{}", buffer);

//         std::thread::sleep(Duration::from_millis(50));
//     }
// }

impl Poblation {

    pub fn run_debug(&mut self) {

        let killers = self.entities.iter()
            .filter(|e| e.is_killer()).map(|e| trunc_uuid(&e.id))
            .collect::<Vec<String>>()
        ;

        let mut moves = Vec::new();
        
        println!("\nInitial state");
        println!("Killers: {:?}", killers);

        for iteration in 1..=N_ITERATIONS {
            
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
                        moves.push(format!("E{} killed E{}", trunc_uuid(&self.entities[i].id), trunc_uuid(&self.entities[j].id)));

                    } else if !self.entities[i].is_killer() && self.entities[j].is_killer() {
                        dead_entities.push(i);
                        self.entities[j].position = Some(entity_next_pos);
                        moves.push(format!("E{} killed E{}", trunc_uuid(&self.entities[j].id), trunc_uuid(&self.entities[i].id)));

                    } else if self.entities[i].is_killer() && self.entities[j].is_killer() {
                        dead_entities.push(i); dead_entities.push(j);
                        moves.push(format!("E{} and E{} killed each other", trunc_uuid(&self.entities[i].id), trunc_uuid(&self.entities[j].id)));
                    }
                
                } else {
                    self.entities[i].position = Some(entity_next_pos);
                }
                
                for &i in dead_entities.iter() {
                    self.entities[i].alive = false;
                }
            }

            println!("Iteration {}", iteration);
            self.show_debug();
            println!("Moves: {:?}\n", moves);

            moves.clear();
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
