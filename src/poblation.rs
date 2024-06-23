
use std::time::Duration;

use rand::Rng;
use colored::*;

use crate::position::*;
use crate::constants::*;
use crate::random::random;
use crate::entity::Entity;
use crate::utils::{cumulative, normalize, trunc_uuid};

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
    pub history: Vec<Vec<Entity>>
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

        Poblation { entities, history: Vec::new() }
    }

    pub fn assign_positions(&self, entities: &mut Vec<Entity>) {

        let mut i = 0;

        while i < entities.len() {

            let random_row = random().gen_range(0..DIMENSIONS.0) as isize;
            let random_col = random().gen_range(0..=1) as isize;
            
            let new_pos = Point::new(random_col, random_row);

            if entities.iter().any(|e| e.position.is_some() && e.get_position() == new_pos) {
                continue
            }

            entities[i].position = Position::Some(new_pos);

            i += 1;
        }
    }

    pub fn selection(&self, final_entities: Vec<Entity>) -> Vec<Entity> {

        // Se crea un vector de nuevas entidades vacio
        
        let mut entities = final_entities;
        let mut new_entities: Vec<Entity> = Vec::new();

        // Mientras la cantidad de entidades no sea igual a la cantidad de entidades
        // Se completa el vector de entidades con entidades aleatorias

        // Estas entidades tienen un valor default de fitness máximo, Añadirlas al final
        // no representará un mayor cambio en el fitness total de la población ya que la probabilidad
        // de que sean seleccionadas como padres será muy baja

        while entities.len() != DIMENSIONS.0 {
            entities.push(Entity::new(Position::None));
        }

        // Se calculan las probabilidades de selección de las entidades

        let mut probs: Vec<f64> = vec![P];

        for i in 1..DIMENSIONS.0 {
            probs.push(P*(1 as f64 - P).powi(i as i32));
        }

        // Se normalizan las probabilidades

        probs = normalize(&probs);

        let cumulative = cumulative(&probs);

        // Se inicia el proceso de creación de las nuevas entidades

        while new_entities.len() != SAMPLE {

            // Obtenemos 2 probabilidades aleatorias, no pueden ser iguales
            
            let prob_1 = random().gen::<f64>();
            let mut prob_2 = random().gen::<f64>();

            while prob_1 == prob_2 {
                prob_2 = random().gen::<f64>();
            }

            // Obtener los indices de las entidades que se cruzarán en base a las probabilidades
            // aleatorias generadas, nuevamente no pueden ser iguales

            let c1_index = cumulative.iter().position(|&p| p > prob_1).unwrap();
            let mut c2_index = cumulative.iter().position(|&p| p > prob_2).unwrap();

            while c1_index == c2_index {
                prob_2 = random().gen::<f64>();
                c2_index = cumulative.iter().position(|&p| p > prob_2).unwrap();
            }

            // Se obtienen las entidades que se cruzarán y se les aplican los operadores de cruce

            let childrens = entities[c1_index].clone() + entities[c2_index].clone();

            // Se añaden las nuevas entidades al vector de nuevas entidades

            new_entities.push(childrens.0);
            new_entities.push(childrens.1);
        }

        // Finalmente se asignan las posiciones a las nuevas entidades 
        // y se retorna el vector de nuevas entidades

        self.assign_positions(&mut new_entities);

        new_entities
    }

    pub fn run(&mut self) {

        let mut generation = 1;

        while generation <= N_GENERATIONS {

            let mut on_goal_entities: Vec<Entity> = Vec::new();

            // Iteramos en la cantidad de iteraciones (movimientos por entidad)
    
            for iteration in 1..=N_ITERATIONS {

                // Vector de entidades muertas en la iteración

                let mut dead_entities: Vec<usize> = Vec::new();

                // Iteramos en las entidades de la población actual
    
                for i in 0..self.entities.len() {
    
                    if !self.entities[i].alive { continue }

                    // Si la entidad actual está en el vector de la meta, continue
    
                    if on_goal_entities.iter().any(|e| e.id == self.entities[i].id) { 
                        continue 
                    }

                    // Si la entidad actual está en la meta, la agregamos al vector de la meta
    
                    if self.entities[i].get_position().x == DIMENSIONS.1 as isize - 1 {
                        on_goal_entities.push(self.entities[i].clone());
                        continue
                    }

                    // Calculamos la siguiente posición de la entidad actual
    
                    let entity_next_pos: Point = self.entities[i].next_position();

                    // Buscamos si hay otra entidad en la siguiente posición de la entidad actual
    
                    let next_pos_index: Option<usize> = self.entities.iter().position(
                        |e| e.get_position() == entity_next_pos && e.alive && e.id != self.entities[i].id
                    );

                    // next_pos_index es un indice opcional, si es Some, entonces hay una entidad en 
                    // la siguiente posición, por lo tanto deberemos comprobar las condiciones de asesinato

                    if let Some(j) = next_pos_index {
    
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

                    // Por otro lado si no hay entidad en la siguiente posición, entonces la entidad
                    // actual se mueve a la siguiente posición

                    // Se recorren las entidades muertas y se les asigna el estado de no viva
                    
                    for &i in dead_entities.iter() {
                        self.entities[i].alive = false
                    }
                }
                
                self.history.push(self.entities.clone());

                // Mostrar la población cada SHOW_THRESHOLD generaciones

                if generation % SHOW_THRESHOLD == 0 {
                    self.show(generation, iteration, None)
                }
            }

            // Ordenar las entidades finales por su fitness (menor a mayor)

            on_goal_entities.sort_by(|a, b| a.fitness.cmp(&b.fitness));

            // Si la cantidad de entidades en la meta es igual a la cantidad de entidades,
            // hay una convergencia, por lo tanto se muestra el resultado y se termina la simulación

            if on_goal_entities.len() == DIMENSIONS.0 {

                for i in 0..self.history.len() {
                    self.show(generation, i + 1, Some(&self.history[i]));
                }

                break
            }

            // De lo contrario, se realiza la selección de las entidades finales

            self.history.clear();
            self.entities = self.selection(on_goal_entities);

            // Y se avanza a la siguiente generación

            generation += 1;
        }

        return
    }

    pub fn show(&self, n_generation: usize, n_iteration: usize, history: Option<&Vec<Entity>>) {

        let entities = match history {
            Some(h) => h,
            None => &self.entities
        };

        let mut buffer = String::new();
        let total_width = DIMENSIONS.1 * 7;

        buffer.push_str(&format!("\x1B[2J\x1B[1;1H"));
        buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));

        let header = format!("| Generation: {:<5} Movement: {:<5} {}|", n_generation, n_iteration, " ".repeat(total_width - 36));
        buffer.push_str(&format!("{}\n", header));
        buffer.push_str(&format!("+{:-<1$}+\n", "", total_width));

        for y in 0..DIMENSIONS.0 {

            for _ in 0..3 {

                buffer.push_str("|");

                for x in 0..DIMENSIONS.1 {

                    let current_pos = Point::new(x as isize, y as isize);

                    if let Some(e) = entities.iter().find(|e| e.get_position() == current_pos && e.alive) {

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

        std::thread::sleep(Duration::from_millis(25));
    }
}

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
        println!("+{:-<12}+", "-".repeat(DIMENSIONS.1 * 12));

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
                println!("+{:-<12}+", "-".repeat(DIMENSIONS.1 * 12));
            }
        }

        println!("+{:-<11}+", "-".repeat(DIMENSIONS.1 * 12));
        println!();
    }
}
