use tablero::celda;

#[path = "tablero.rs"]
pub mod tablero;

#[derive(Debug)]
#[derive(Clone)]
pub struct Snake {
    tablero: tablero::Tablero,
    // array de posiciones (fila, columna) de todo el cuerpo de la serpiente (la pos 0 siempre sera la cabeza y por tanto la que manejo con las flechas)
    serpiente: Vec<(i32, i32)>,
    // fila, columna 
    pos_fruta: (i32, i32),
}

impl Snake {
    pub fn new(
        tamano_cuadricula: (usize, usize),
        pos_cabeza_serpiente: (i32, i32),
        pos_fruta: (i32, i32),
    ) -> Self {
        let _pos_serpiente = (pos_cabeza_serpiente.0 as usize, pos_cabeza_serpiente.1 as usize);
        let _pos_fruta = (pos_fruta.0 as usize, pos_fruta.1 as usize);
        let _tablero = tablero::Tablero::new(
            tamano_cuadricula,
            _pos_serpiente,
            _pos_fruta,
        );

        // inicializamos la serpiente pero solo con la cabeza
        let mut serpiente = Vec::new();
        serpiente.push(pos_cabeza_serpiente);

        Snake {
            tablero: _tablero,
            serpiente: serpiente,
            pos_fruta: pos_fruta,
        }
    }

    // generamos una nueva fruta en una posicion aleatoria en la que no haya serpiente
    pub fn update_fruta(&mut self) {
        let (filas, columnas) = self.tablero.get_tamano();
        let mut nueva_pos = (0, 0);
        loop {
            nueva_pos.0 = rand::random::<usize>() % filas;
            nueva_pos.1 = rand::random::<usize>() % columnas;
            if self.tablero.get_celda(nueva_pos.0, nueva_pos.1) == celda::Celda::Vacio {
                break;
            }
        }
        self.tablero.update_celda(
            (nueva_pos.0 as usize, nueva_pos.1 as usize), 
            celda::Celda::Fruta
        );
    }

    // La app que pinte nuestro snake deberá de conocer la disposicion del tablero.
    pub fn get_tablero(&self) -> tablero::Tablero {
        // devuelvo una copia para que la versión original no se vea afectada. esto porque solo se usará para pintar el tablero
        self.tablero.clone()
    }

    // Debo de poder actualizar la posicion de la serpiente
    fn update_pos_serpiente(
        &mut self, 
        nueva_pos_cabeza: (i32, i32)
    ) {
        // debo comprobar si la nueva posicion es valida, si se sale volvera a aparecer por el lado opuesto
        let (filas, columnas) = self.tablero.get_tamano();
        let mut _nueva_pos_cabeza = nueva_pos_cabeza;
        if _nueva_pos_cabeza.0 >= filas as i32 {
            _nueva_pos_cabeza.0 = 0;
        }
        if _nueva_pos_cabeza.1 >= columnas as i32 {
            _nueva_pos_cabeza.1 = 0;
        }
        if _nueva_pos_cabeza.0 < 0 {
            _nueva_pos_cabeza.0 = (filas - 1) as i32;
        }
        if _nueva_pos_cabeza.1 < 0 {
            _nueva_pos_cabeza.1 = (columnas - 1) as i32;
        }

        // comprobamos si la casilla a la que nos dirigimos es una fruta
        let futura_casilla = self.tablero.get_celda(_nueva_pos_cabeza.0 as usize, _nueva_pos_cabeza.1 as usize);

        // si es la fruta alargamos nuestra serpiente en 1 y desplazamos las posiciones del resto del cuerpo (como si creciese desde la cabeza)
        if futura_casilla == celda::Celda::Fruta {
            self.serpiente.insert(0, _nueva_pos_cabeza);
            // generamos una nueva fruta
            self.update_fruta();
            // borramos la fruta y la pintamos de verde
            self.tablero.update_celda(
                (_nueva_pos_cabeza.0 as usize, _nueva_pos_cabeza.1 as usize), 
                celda::Celda::Serpiente
            );
        } else {
            // Inserta la nueva cabeza al inicio del vector
            self.serpiente.insert(0, _nueva_pos_cabeza);
            // Remueve la última posición (cola) para mantener la longitud constante
            let pos_cola = self.serpiente.pop().unwrap();

            // Actualiza el tablero: limpia la celda de la cola y marca la nueva cabeza
            self.tablero.update_celda((pos_cola.0 as usize, pos_cola.1 as usize), celda::Celda::Vacio);
            self.tablero.update_celda((_nueva_pos_cabeza.0 as usize, _nueva_pos_cabeza.1 as usize), celda::Celda::Serpiente);
        }

    }

    // Bajar 1 posicion
    pub fn bajar_pos_serpiente(&mut self) {
        self.update_pos_serpiente(
            (self.serpiente[0].0 + 1, self.serpiente[0].1)
        );
    }
    // Subir 1 posicion
    pub fn subir_pos_serpiente(&mut self) {
        self.update_pos_serpiente(
            (self.serpiente[0].0 - 1, self.serpiente[0].1)
        );
    }
    // Mover a la derecha
    pub fn derecha_pos_serpiente(&mut self) {
        self.update_pos_serpiente(
            (self.serpiente[0].0, self.serpiente[0].1 + 1)
        );
    }
    // Mover a la izquierda
    pub fn izquierda_pos_serpiente(&mut self) {
        self.update_pos_serpiente(
            (self.serpiente[0].0, self.serpiente[0].1 - 1)
        );
    }

}