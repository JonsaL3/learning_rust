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
        // fila, columna
        pos_inicial_serpiente: (usize, usize),
        // fila, columna 
        pos_inicial_fruta: (usize, usize),
    ) -> Self {
        // Construimos el tablero con el tamaño especificado.
        let mut m_tablero: Vec<Vec<celda::Celda>> = vec![vec![celda::Celda::Vacio; tamano_cuadricula.0]; tamano_cuadricula.1];
        // Seteamos la celda de la serpiente y la fruta.
        m_tablero[pos_inicial_serpiente.0][pos_inicial_serpiente.1] = celda::Celda::Serpiente;
        
        // Construimos el tablero.
        Tablero { tablero: m_tablero }
    }

    // fila, columna
    pub fn update_celda(
        &mut self, 
        pos: (usize, usize), 
        nuevo_tipo: celda::Celda
    ) {
        self.tablero[pos.0][pos.1] = nuevo_tipo;
    }

    // La app que pinte nuestro snake deberá de conocer la celda para poder pintarla.
    pub fn get_celda(
        &self, 
        fila: usize, 
        columna: usize
    ) -> celda::Celda {
        // devuelvo una copia para que la versión original no se vea afectada. esto porque solo se usará para pintar el tablero
        self.tablero[fila][columna].clone()
    }

    pub fn get_tamano(&self) -> (usize, usize) {
        (self.tablero.len(), self.tablero[0].len())
    }
}