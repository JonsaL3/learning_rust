#[path = "tablero.rs"]
pub mod tablero;

#[derive(Debug)]
#[derive(Clone)]
pub struct Snake {
    tablero: tablero::Tablero,
    // fila, columna (proximamente un array con todas las posiciones de la serpiente)
    pos_serpiente: (i32, i32),
    // fila, columna 
    pos_fruta: (i32, i32),
}

impl Snake {
    pub fn new(
        tamano_cuadricula: (usize, usize),
        pos_serpiente: (i32, i32),
        pos_fruta: (i32, i32),
    ) -> Self {
        let _pos_serpiente = (pos_serpiente.0 as usize, pos_serpiente.1 as usize);
        let _pos_fruta = (pos_fruta.0 as usize, pos_fruta.1 as usize);
        let _tablero = tablero::Tablero::new(
            tamano_cuadricula,
            _pos_serpiente,
            _pos_fruta,
        );

        Snake {
            tablero: _tablero,
            pos_serpiente: pos_serpiente,
            pos_fruta: pos_fruta,
        }
    }

    // La app que pinte nuestro snake deberá de conocer la disposicion del tablero.
    pub fn get_tablero(&self) -> tablero::Tablero {
        // devuelvo una copia para que la versión original no se vea afectada. esto porque solo se usará para pintar el tablero
        self.tablero.clone()
    }

    // Debo de poder actualizar la posicion de la serpiente
    fn update_pos_serpiente(
        &mut self, 
        nueva_pos: (i32, i32)
    ) {
        // debo comprobar si la nueva posicion es valida, si se sale volvera a aparecer por el lado opuesto
        let (filas, columnas) = self.tablero.get_tamano();
        let mut nueva_pos = nueva_pos;
        if nueva_pos.0 >= filas as i32 {
            nueva_pos.0 = 0;
        }
        if nueva_pos.1 >= columnas as i32 {
            nueva_pos.1 = 0;
        }
        if nueva_pos.0 < 0 {
            nueva_pos.0 = (filas - 1) as i32;
        }
        if nueva_pos.1 < 0 {
            nueva_pos.1 = (columnas - 1) as i32;
        }

        // Actualizo la celda de la serpiente
        self.tablero.update_celda(
            (self.pos_serpiente.0 as usize, self.pos_serpiente.1 as usize), 
            tablero::celda::Celda::Vacio
        );

        // Actualizo la posicion de la serpiente
        self.pos_serpiente = nueva_pos;

        // Actualizo la celda de la serpiente
        self.tablero.update_celda(
            (self.pos_serpiente.0 as usize, self.pos_serpiente.1 as usize), 
            tablero::celda::Celda::Serpiente
        );
    }

    // Bajar 1 posicion
    pub fn bajar_pos_serpiente(&mut self) {
        self.update_pos_serpiente((self.pos_serpiente.0 + 1, self.pos_serpiente.1));
    }
    // Subir 1 posicion
    pub fn subir_pos_serpiente(&mut self) {
        self.update_pos_serpiente((self.pos_serpiente.0 - 1, self.pos_serpiente.1));
    }
    // Mover a la derecha
    pub fn derecha_pos_serpiente(&mut self) {
        self.update_pos_serpiente((self.pos_serpiente.0, self.pos_serpiente.1 + 1));
    }
    // Mover a la izquierda
    pub fn izquierda_pos_serpiente(&mut self) {
        self.update_pos_serpiente((self.pos_serpiente.0, self.pos_serpiente.1 - 1));
    }

}