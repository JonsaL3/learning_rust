struct Persona {
    nombre: String,
    edad: i32,
}

impl Persona {
    // funciones del "objeto"
    fn calcular_doble_edad(&self) -> i32 {
        self.edad * 2
    }
    fn obtener_longitud_nombre(&self) -> i32 {
        self.nombre.len() as i32
    }
    // funciones "estaticas"
    fn obtener_cubo(x: i32) -> i32 {
        x*x*x
    }
}

struct Coordenadas(i32, i32, i32);

pub fn main_structures() {
    let persona_inmutable: Persona = Persona {
        nombre: String::from("Gonzalo"),
        edad: 2323
    };
    let mut persona_mutable: Persona = Persona {
        nombre: String::from("Gonzalo"),
        edad: 24
    };
    let persona_derivada: Persona = Persona { 
        nombre: String::from("Estebanco"),
        ..persona_inmutable
    };
    persona_mutable.edad = 512;
    println!("INMUTABLE: {}, {} MUTABLE: {}, {}", persona_inmutable.nombre, persona_inmutable.edad, persona_mutable.nombre, persona_mutable.edad);

    println!("persona derivada {}, {}", persona_derivada.nombre, persona_derivada.edad);

    let tupla_coordenadas: Coordenadas = Coordenadas(256, 512, 1024);
    println!("Tupla {} {} {}", tupla_coordenadas.0, tupla_coordenadas.1, tupla_coordenadas.2);

    let cubo: i32 = Persona::obtener_cubo(9);
    println!("el valor del cubo es de: {}", cubo);

    println!("El doble de la edad es {} y la longitud del nombre es de {}", persona_inmutable.calcular_doble_edad(), persona_inmutable.obtener_longitud_nombre())
}