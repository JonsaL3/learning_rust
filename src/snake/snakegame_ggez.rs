#[path = "snake.rs"]
pub mod snake;

#[path = "tablero.rs"]
pub mod tablero;

#[path = "celda.rs"]
pub mod celda;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};
use snake::Snake as Snake;

/**
 * Estados de lo que hay en pantalla, todo lo que compone nuestro juego
 */
struct SnakeGameGGEZ {
    // Logica del juego de la serpiente
    snake: Snake,
    // Tamaños en pantalla
    tamano_celda: f32,
    // Conteo de ticks
    acumulador: f32,
    intervalo: f32,
}

impl SnakeGameGGEZ {
    fn new() -> GameResult<SnakeGameGGEZ> {
        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for SnakeGameGGEZ {
    /**
     * Esto es lo que es en unity la funcion update, hace la misma cosa
     */
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        // Obtenemos el tiempo transcurrido
        let dt = _ctx.time.delta().as_secs_f32();

        // lo vamos acumulando
        self.acumulador += dt;
    
        // si el tiempo transcurrido es igual a nuestro intervalo (0.3s en nuestro caso) ejecutamos un tick
        if self.acumulador >= self.intervalo {
            // aqui es donde se ejecuta el tick
            println!("Tick!:");

            // todo aqui iran cosas

            // reseteamos el acumulador
            self.acumulador = 0.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        // Creamos en el lienzo en el que vamos a pintar todo.
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        // y pintamos el tablero en el estado en el que se encuentre
        draw_tablero(ctx, &mut canvas, self.snake.get_tablero(), self.tamano_celda)?;

        // Pintamos el canvas
        canvas.finish(ctx)
    }

    /*
     * Handleo de teclas
     */
    /*fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Down) => {
                println!("Tecla abajo presionada.");
                self.direccion_serpiente = Direccion::Abajo;
            }
            Some(KeyCode::Up) => {
                println!("Tecla arriba presionada.");
                self.direccion_serpiente = Direccion::Arriba;
            }
            Some(KeyCode::Left) => {
                println!("Tecla izquierda presionada.");
                self.direccion_serpiente = Direccion::Izquierda;
            }
            Some(KeyCode::Right) => {
                println!("Tecla derecha presionada.");
                self.direccion_serpiente = Direccion::Derecha;
            }
            // None no hago nada
            _ => (),
        }
        Ok(())
    }*/
}

/**
 * Pintamos el tablero
 */
fn draw_tablero(
    ctx: &mut Context, 
    canvas: &mut graphics::Canvas, 
    tablero: tablero::Tablero, 
    tamano_celda: f32
) -> GameResult {
    
}

/**
 * Pintamos una celda
 */
fn draw_celda(
    ctx: &mut Context, 
    canvas: &mut graphics::Canvas,
    tamano_celda: f32,
    canvas_pos_x: f32,
    canvas_pos_y: f32,
    celda: celda::Celda,
) -> GameResult {
    // En funcion del tipo de celda la pintamos de un color o de otro
    let color: Color;
    match celda {
        celda::Celda::Vacio => {
            color = Color::WHITE;
        }
        celda::Celda::Fruta => {
            color = Color::RED;
        }
        celda::Celda::Serpiente => {
            color = Color::GREEN;
        }
    }
    // Construimos el uadrado y lo pintamos en las coordenadas concretas del canvas con el tamaño
    // especificado -1 para ver la cuadricula.
    let rect = graphics::Rect::new(tamano_celda-1.0, tamano_celda-1.0, tamano_celda-1.0, tamano_celda-1.0);
    let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
    canvas.draw(&mesh, Vec2::new(canvas_pos_x, canvas_pos_y));
    Ok(())
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
    