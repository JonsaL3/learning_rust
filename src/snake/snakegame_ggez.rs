#[path = "snake.rs"]
mod snake;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use ggez::input::keyboard::{KeyCode, KeyInput};

/**
 * Estados de lo que hay en pantalla, todo lo que compone nuestro juego, estados, etc
 */
struct SnakeGameGGEZ {
    // Logica del juego de la serpiente
    snake: snake::Snake,
    // Tamaños en pantalla
    tamano_celda: f32,
    // Ultima tecla pulsada
    ultima_tecla_pulsada: Option<KeyCode>,
    // Conteo de ticks
    acumulador: f32,
    intervalo: f32,
}

// funciones especificas de nuestro juego que usan los parametros del estado declarado arriba
impl SnakeGameGGEZ {
    fn new() -> GameResult<SnakeGameGGEZ> {
        let _snake_game = SnakeGameGGEZ {
            snake: snake::Snake::new((10, 10), (5, 5), (2, 2)),
            tamano_celda: 20.0,
            ultima_tecla_pulsada: Some(KeyCode::Down), // Por defecto bajara la serpiente),
            acumulador: 0.0,
            intervalo: 0.1,
        };
        Ok(_snake_game)
    }
    
    /**
     * Pintamos una celda
     */
    fn draw_celda(
        &self,
        ctx: &mut Context, 
        canvas: &mut graphics::Canvas,
        canvas_pos_x: f32,
        canvas_pos_y: f32,
        celda: snake::tablero::celda::Celda,
    ) -> GameResult {
        // En funcion del tipo de celda la pintamos de un color o de otro
        let color: Color;
        match celda {
            snake::tablero::celda::Celda::Vacio => {
                color = Color::WHITE;
            }
            snake::tablero::celda::Celda::Fruta => {
                color = Color::RED;
            }
            snake::tablero::celda::Celda::Serpiente => {
                color = Color::GREEN;
            }
        }

        // Construimos el uadrado y lo pintamos en las coordenadas concretas del canvas con el tamaño
        // especificado -1 para ver la cuadricula.
        let rect = graphics::Rect::new(
            self.tamano_celda - 1.0, 
            self.tamano_celda - 1.0,
            self.tamano_celda - 1.0, 
            self.tamano_celda - 1.0
        );

        let mesh = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            rect, 
            color
        )?;

        canvas.draw(
            &mesh, 
            Vec2::new(canvas_pos_x, canvas_pos_y)
        );

        Ok(())
    }


    /**
     * Pintamos el tablero
     */
    pub fn draw_tablero(
        &self,
        ctx: &mut Context, 
        canvas: &mut graphics::Canvas, 
        tamano_celda: f32
    ) -> GameResult {

        // Obtenemos el tablero
        let tablero = self.snake.get_tablero();
        // Obtenemos el tamaño del tablero
        let (filas, columnas) = tablero.get_tamano();

        // Calculamos el tamaño del canvas
        let canvas_pos_x = 0.0;
        let canvas_pos_y = 0.0;

        // Pintamos cada celda
        for i in 0..filas {
            for j in 0..columnas {
                // Obtenemos la celda
                let celda = tablero.get_celda(i, j);
                // Pintamos la celda
                self.draw_celda(
                    ctx, 
                    canvas, 
                    canvas_pos_x + (j as f32 * tamano_celda) - 1.0, 
                    canvas_pos_y + (i as f32 * tamano_celda) - 1.0, 
                    celda
                )?;
            }
        }
        Ok(())
    }
}

// Trait especifico del mundo de ggez, para manejar eventos y dibujar cosas
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
            // obtenemos los fps para pintarlos cada tick por consola
            // Aquí puedes, por ejemplo, imprimir los FPS en la consola
            let current_fps = _ctx.time.fps();
            println!("Tick .. FPS: {}", current_fps);

            // todo aqui iran cosas
            match self.ultima_tecla_pulsada {
                Some(key) => {
                    // Si la tecla pulsada es una de las teclas de dirección, movemos la serpiente
                    if key == KeyCode::Up {
                        self.snake.subir_pos_serpiente();
                    } else if key == KeyCode::Down {
                        self.snake.bajar_pos_serpiente();
                    } else if key == KeyCode::Left {
                        self.snake.izquierda_pos_serpiente();
                    } else if key == KeyCode::Right {
                        self.snake.derecha_pos_serpiente();
                    }
                }
                None => {}
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

        // y pintamos el tablero en el estado en el que se encuentre
        self.draw_tablero(ctx, &mut canvas, self.tamano_celda)?;

        // Pintamos el canvas
        canvas.finish(ctx)
    }

    /*
     * Handleo de teclas
     */
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        // Si pulsamos alguna de las teclas de dirección modificamos el estado de manera "reactiva"
        if let Some(key) = input.keycode {
            if key == KeyCode::Up || key == KeyCode::Down || key == KeyCode::Left || key == KeyCode::Right {
                self.ultima_tecla_pulsada = input.keycode;
            }
        }
        Ok(())
    }

}

/**
 * Nuestra funcion main lo que hace realmente es comenzar el looper
 */
pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("mi jueguito", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = SnakeGameGGEZ::new()?;
    event::run(ctx, event_loop, state)
}
    