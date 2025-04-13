use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;

/**
 * Nuestro juego 
 */
struct SnakeState {
    numero_columnas: i32,
    numero_filas: i32
}

impl SnakeState {
    fn new() -> GameResult<SnakeState> {
        let s = SnakeState {
            numero_columnas: 32,
            numero_filas: 32
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for SnakeState {
    /**
     * Esto es lo que es en unity la funcion update, hace la misma cosa
     */
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /**
     * Esto digamos cutremente que es el equivalente a la funcion start de unity
     */
    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        // Creamos en el lienzo en el que vamos a pintar todo.
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        // Comenzamos a definir los elementos que componen nuestro canvas

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            10.0,
            1.0,
            Color::WHITE,
        )?;

        for y_iterator in 1..24 {
            let y = y_iterator as f32;
            for x_iterator in 1..24 {
                let x = x_iterator as f32;
                canvas.draw(&circle, Vec2::new(10.0 * x * 2 as f32, 10.0 * y * 2 as f32));
            }
        }

        // Pintamos el canvas
        canvas.finish(ctx)
    }
}

/**
 * Nuestra funcion main lo que hace realmente es comenzar el looper
 */
pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("mi jueguito", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = SnakeState::new()?;
    event::run(ctx, event_loop, state)
}
    