
// use ggez::{
//     event::{self, EventHandler},
//     graphics::{self, Color, Rect},
//     Context, GameResult,
// };

// const GRID_SIZE: (i16, i16) = (20, 10);
// const GRID_CELL_SIZE: (i16, i16) = (40, 40);
// const GRID_PADDING: (f32, f32) = (50.0, 50.0); // Padding de 50 unidades en ambos ejes
// const GRID_OFFSET: (f32, f32) = (GRID_PADDING.0 / 2.0, GRID_PADDING.1 / 2.0);
// const SCREEN_SIZE: (f32, f32) = (
//     GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32 + GRID_PADDING.0,
//     GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32 + GRID_PADDING.1,
// );

// struct GameState {
//     points: Vec<(u8, u8)>,
// }

// impl GameState {
    
//     fn new() -> Self {

//         let points = vec![
//             (0, 0),
//             (1, 1),
//         ];

//         GameState { points }
//     }
// }

// impl EventHandler for GameState {

//     fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {

//         for i in 0..self.points.len() {
//             let (x, y) = self.points[i];
//             self.points[i] = (x + 1, y + 1);
//         }

//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {

//         let mut canvas = graphics::Canvas::from_frame(
//             ctx, Color::BLACK
//         );

//         // Dibujar la grilla con el padding
//         for i in 0..=GRID_SIZE.0 {
//             for j in 0..=GRID_SIZE.1 {
//                 let x = i as f32 * GRID_CELL_SIZE.0 as f32 + GRID_OFFSET.0;
//                 let y = j as f32 * GRID_CELL_SIZE.1 as f32 + GRID_OFFSET.1;

//                 // Dibujar líneas horizontales
//                 let points = vec![
//                     [GRID_OFFSET.0, y],
//                     [GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32 + GRID_OFFSET.0, y],
//                 ];
//                 let line = graphics::Mesh::new_line(ctx, &points, 1.0, Color::WHITE)?;
//                 graphics::draw(&mut canvas, &line, graphics::DrawParam::default());

//                 // Dibujar líneas verticales
//                 let points = vec![
//                     [x, GRID_OFFSET.1],
//                     [x, GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32 + GRID_OFFSET.1],
//                 ];
//                 let line = graphics::Mesh::new_line(ctx, &points, 1.0, Color::WHITE)?;
//                 graphics::draw(&mut canvas, &line, graphics::DrawParam::default());
//             }
//         }

//         // Dibujar los puntos

//         for (i, j) in &self.points {
//             let x = *i as f32 * GRID_CELL_SIZE.0 as f32 + GRID_OFFSET.0;
//             let y = *j as f32 * GRID_CELL_SIZE.1 as f32 + GRID_OFFSET.1;

//             let rectangle = graphics::Mesh::new_rectangle(
//                 ctx,
//                 graphics::DrawMode::fill(),
//                 Rect::new(x, y, GRID_CELL_SIZE.0 as f32, GRID_CELL_SIZE.1 as f32),
//                 Color::WHITE,
//             )?;
//             graphics::draw(&mut canvas, &rectangle, graphics::DrawParam::default());
//         }

//         canvas.finish(ctx)?;

//         Ok(())
//     }
// }

// fn main() -> GameResult {
//     let (ctx, event_loop) = ggez::ContextBuilder::new("Genetic", "Lr. Nv")
//         .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
//         .build()?;
//     let state = GameState::new();
//     event::run(ctx, event_loop, state)
// }