#[path = "celda.rs"]
pub mod celda;

#[derive(Debug)]
#[derive(Clone)]
pub struct Tablero {
    tablero: Vec<Vec<celda::Celda>>
}

impl Tablero {
    pub fn new(
        // numero_filas, numero_columnas
        tamano_cuadricula: (usize, usize),
    ) -> Self {
        let m_tablero: Vec<Vec<celda::Celda>> = vec![vec![celda::Celda::Vacio; tamano_cuadricula.0]; tamano_cuadricula.1];
        // Construimos el tablero.
        Tablero {  tablero: m_tablero }
    }

    // fila, columna
    pub fn update_celda(
        &mut self, 
        pos: (usize, usize), 
        nuevo_tipo: celda::Celda
    ) {
        self.tablero[pos.0][pos.1] = nuevo_tipo;
    }
}