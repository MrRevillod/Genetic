
# Algoritmo genetico

En esta implementación buscamos encontrar un punto de convergencia de una generación, o sea donde la columna de 
la meta del tablero sea poblada por completo. Para esto se implementa un algoritmo genético que se encarga de
trabajar y cruzar con las entidades de mejor fitness (menor cantidad de movimientos) para encontrar la mejor solución.

## Clases / Structs del algoritmo

### Point Struct

La struct Point como su nombre lo indica es un punto en un plano cartesiano. Fue creada para facilitar la representación de los puntos en el algoritmo genético.

```rust
pub struct Punto {
    x: isize,
    y: isize,
}
```

#### Implementaciones de la struct

La principal implementación de la struct es un vector de coordenadas direccionales que se utilizan para moverse en el plano cartesiano.

```rust
pub static ref DIRECTIONS: [Point; 8] = [
    Point::new(-1, -1), // up-left
    Point::new(0, -1),  // up
    Point::new(1, -1),  // up-right
    Point::new(1, 0),   // right
    Point::new(1, 1),   // down-right
    Point::new(0, 1),   // down
    Point::new(-1, 1),  // down-left
    Point::new(-1, 0)   // left
];
```

### Entity Struct

La struct Entity representa una entidad en el algoritmo genético. Cada entidad tiene un cromosoma llamado *values*, un fitness que es un valor entero que indica la calidad de la entidad. Un indicador booleano de si la entidad es asesina, un indicador booleano de si la entidad está viva, una posición en el tablero y un color.

```rust
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
```

El vector **values** es un vector de **f64** normalizado, con 8 elementos que representan las 8 direcciones posibles en el tablero. La asignación de los valores es completamente aleatoria. De igual manera el gen asesino es asignado aleatoriamente. Existe una probabilidad del 5% de que una entidad sea asesina.

#### Métodos de la struct

El método más significativo excluyendo sus constructores  **new** y **from** es el método **next_position** que se encarga 
de obtener la siguiente posición de la entidad en el tablero (retorna un **Point**). Dependiento de esta posición se actualiza el fitness de la entidad.


```rust
pub fn next_position(&mut self) -> Point {

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
        return current_pos
    }

    // Calculate the next position and verify the limits
    // negatives and greater than the row limit

    let next_pos = current_pos + dir;

    // Check if next_pos is outside the limits

    if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= DIMENSIONS.1 as isize || next_pos.y >= DIMENSIONS.0 as isize {
        return current_pos
    }

    // Update the position and the fitness of the entity

    self.fitness -= 1;
    next_pos
}
```

Por otro lado, esta struct presenta una implementación del operador **+** el cual cruzará dos entidades. Este operador se encarga de cruzar los cromosomas de dos entidades y generar un par de entidades con sus valores mezclados.

Con c1 como padre y c2 como madre, se cruzarán ambas entidades tal que: 

Lo principal es dividir el cromosoma (values) por la mitad y cruzar los valores de cada mitad de esta manera:

```rust
c1 + c2 = (hijo1, hijo2)
```

Entonces, si tenemos dos cromosomas de la siguiente forma:

```rust
c1_1 = values[0..4] y c1_2 = values[4..8]
c2_1 = values[0..4] y c2_2 = values[4..8]

hijo1_values = [c1_1, c2_2]
hijo2_values = [c2_1, c1_2]

hijo_1_killer = c2_killer
hijo_2_killer = c1_killer
```

Además existe una probabilidad de mutación, la cual es del 5% y se encarga de cambiar un valor aleatorio del cromosoma. Es importante destacar que un valor puede mutar a un valor menor o mayor.

### Poblation Struct

La struct Poblation representa una población de entidades en el algoritmo genético. Cada población tiene un vector de entidades, y otros relacionados a las estadísticas de la población.

```rust
#[derive(Clone, Debug)]
pub struct Poblation {
    pub entities: Vec<Entity>,
}
```

#### Métodos de la struct

##### Constructor de la struct

El constructor de la struct crea un vector de entidades. Itera en un rango dado por la constante SAMPLE y por cada iteración crea una entidad con una posición aleatoria. Si la posición ya está ocupada por otra entidad, se omite la creación de la entidad. De esta manera se garantiza que no existan entidades en la misma posición.

```rust
pub fn new() -> Self {

    let mut entities: Vec<Entity> = Vec::new();

    let mut i = SAMPLE;

    while i > 0 {

        let random_row = random().gen_range(0..DIMENSIONS.0) as isize;
        let random_col = random().gen_range(0..=1) as isize;

        let new_pos = Point::new(random_col, random_row);

        if entities.iter().any(|e| e.get_position() == new_pos) {
            continue
        }

        entities.push(Entity::new(Position::Some(new_pos)));

        i -= 1;
    }

    Poblation { entities }
}
```

##### Método **run**

El método run ejecuta la logica de manejo del tablero, interacción y posiciones entre entidades. 

```rust
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
        // Y se avanza a la siguiente generación
        
        self.entities = self.selection(on_goal_entities);
        generation += 1;
    }

    return
}
```
