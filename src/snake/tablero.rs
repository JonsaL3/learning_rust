#[path = "celda.rs"]
pub mod celda;

#[derive(Debug)]
pub struct Tablero {
    current_pos_serpiente: (usize, usize),
    tablero: Vec<Vec<celda::Celda>>
}

impl Tablero {
    /**
     * Constructor (?)
     */
    pub fn new(
        n_filas: usize, 
        n_columnas: usize,
        pos_inicial_serpiente: (usize, usize)
    ) -> Self {
        let mut tablero: Vec<Vec<celda::Celda>> = Vec::new();
        for i in 0..n_columnas {
            let mut fila: Vec<celda::Celda> = Vec::new();
            for j in 0..n_filas {
                if i == pos_inicial_serpiente.0 && j == pos_inicial_serpiente.1 {
                    fila.push(celda::Celda::new(0, 255, 0));
                } else {
                    fila.push(celda::Celda::new(255, 255, 255));
                }
            }
            tablero.push(fila);
        }
        Tablero { 
            tablero: tablero, 
            current_pos_serpiente: pos_inicial_serpiente
        }
    }

    /**
     * getters
     */
    pub fn get_filas(&self) -> usize {
        self.tablero[0].len()
    }

    pub fn get_columnas(&self) -> usize {
        self.tablero.len()
    }

    pub fn get_celda(&self, pos_x: usize, pos_y: usize) -> celda::Celda {
        if pos_x < 0 || pos_x >= self.get_columnas()|| pos_y < 0 || pos_y >= self.get_filas() {
            panic!("Posicion fuera de rango");
        }
        self.tablero[pos_x as usize][pos_y as usize].clone()
    }

    /**
     * Si la serpiente se mueve a una posicion fuera del tablero, aparecera por el lado contrario
     */
    fn mover_serpiente(
        &mut self, 
        direccion: (i32, i32)
    ) {
        let mut new_pos_x = self.current_pos_serpiente.0 as i32 + direccion.0;
        let mut new_pos_y = self.current_pos_serpiente.1 as i32 + direccion.1;

        if new_pos_x < 0 {
            new_pos_x = self.get_columnas() as i32 - 1;
        } else if new_pos_x >= self.get_columnas() as i32 {
            new_pos_x = 0;
        }

        if new_pos_y < 0 {
            new_pos_y = self.get_filas() as i32 - 1;
        } else if new_pos_y >= self.get_filas() as i32 {
            new_pos_y = 0;
        }

        self.tablero[self.current_pos_serpiente.0][self.current_pos_serpiente.1] = celda::Celda::new(255, 255, 255);
        self.tablero[new_pos_x as usize][new_pos_y as usize] = celda::Celda::new(0, 255, 0);
        self.current_pos_serpiente = (new_pos_x as usize, new_pos_y as usize);
    }

    pub fn mover_serpiente_abajo(&mut self) {
        // en ggez - es subir y + es bajar, porque 0,0 es la esquina superior izquierda
        self.mover_serpiente((0, 1));
    }

    pub fn mover_serpiente_arriba(&mut self) {
        // en ggez - es subir y + es bajar, porque 0,0 es la esquina superior izquierda
        self.mover_serpiente((0, -1));
    }

    pub fn mover_serpiente_izquierda(&mut self) {
        // en ggez - es subir y + es bajar, porque 0,0 es la esquina superior izquierda
        self.mover_serpiente((-1, 0));
    }

    pub fn mover_serpiente_derecha(&mut self) {
        // en ggez - es subir y + es bajar, porque 0,0 es la esquina superior izquierda
        self.mover_serpiente((1, 0));
    }
}