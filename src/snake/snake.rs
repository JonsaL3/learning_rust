use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use tablero::Tablero;
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};

#[path = "tablero.rs"]
mod tablero;
/**
 * Nuestro juego 
 */
struct SnakeState {
    tamano_celda: f32,
    acumulador: f32,
    intervalo: f32,
    tablero: Tablero,
    direccion_serpiente: Direccion,
}

enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha
}

impl SnakeState {
    fn new() -> GameResult<SnakeState> {
        // Construimos un tablero con los datos definidos en el state
        let tablero = Tablero::new(12, 12, (0,0));
        let s = SnakeState {
            tamano_celda: 24.0,
            acumulador: 0.0, // aqui acumulamos cuanto tardamos en llegar a 0.3 para saber si debemos volver a actualizar
            intervalo: 0.1, // Intervalo de tiempo entre cada movimiento, para no actualizar constantemente.
            tablero: tablero,
            direccion_serpiente: Direccion::Abajo, // por defecto, la serpiente irá hacia abajo
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for SnakeState {
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

            // podemos interpretar los ticks como frecuencia de refresco del tablerom cuya frecuencia de refresco es de 0.3s
            match self.direccion_serpiente {
                Direccion::Abajo => {
                    self.tablero.mover_serpiente_abajo();
                }
                Direccion::Arriba => {
                    self.tablero.mover_serpiente_arriba();
                }
                Direccion::Izquierda => {
                    self.tablero.mover_serpiente_izquierda();
                }
                Direccion::Derecha => {
                    self.tablero.mover_serpiente_derecha();
                }
            }

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

        // y lo pintamos, por defecto será todo blanco
        draw_tablero(ctx, &mut canvas, &self.tablero, self.tamano_celda)?;

        // Pintamos el canvas
        canvas.finish(ctx)
    }

    /**
     * Handleo de teclas
     */
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
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
    }
}

/**
 * Pintamos el tablero
 */
fn draw_tablero(
    ctx: &mut Context, 
    canvas: &mut graphics::Canvas, 
    tablero: &Tablero, 
    tamano_celda: f32
) -> GameResult {
    let mut canvas_pos_x = 0.0;
    let mut canvas_pos_y = 0.0;
    for i in 0..tablero.get_columnas() {
        for j in 0..tablero.get_filas() {
            let m_celda = tablero.get_celda(i, j);
            draw_celda(ctx, canvas, tamano_celda, &m_celda, canvas_pos_x, canvas_pos_y)?;
            canvas_pos_x += tamano_celda;
        }
        canvas_pos_x = 0.0;
        canvas_pos_y += tamano_celda;
    }
    Ok(())
} pintar la posicion de cada casilla para entender como se esta dibujando el array, tienes el problema de ejes de coordenadas invertidos

/**
 * Pintamos una celda
 */
fn draw_celda(
    ctx: &mut Context, 
    canvas: &mut graphics::Canvas,
    tamano_celda: f32,
    celda: &tablero::celda::Celda,
    canvas_pos_x: f32,
    canvas_pos_y: f32
) -> GameResult {
    let color = Color::from_rgb(celda.r_color(), celda.g_color(), celda.b_color());
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
    