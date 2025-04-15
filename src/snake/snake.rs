#[path = "tablero.rs"]
pub mod tablero;

#[derive(Debug)]
#[derive(Clone)]
pub struct Snake {
    tablero: tablero::Tablero,
    // fila, columna (proximamente un array con todas las posiciones de la serpiente)
    pos_serpiente: (usize, usize),
    // fila, columna 
    pos_fruta: (usize, usize),
}

impl Snake {
    pub fn new(
        tamano_cuadricula: (usize, usize),
        pos_serpiente: (usize, usize),
        pos_fruta: (usize, usize),
    ) -> Self {
        let _tablero = tablero::Tablero::new(tamano_cuadricula);

        // todo pintar los distintos elementos.

        Snake {
            tablero: _tablero,
            pos_serpiente: (5, 5),
            pos_fruta: (2,2),
        }
    }

    // La app que pinte nuestro snake deberá de conocer la disposicion del tablero.
    pub fn get_tablero(&self) -> &tablero::Tablero {
        &self.tablero
    }
}