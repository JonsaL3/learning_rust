use std::io;

/**
 * Antes de programar el maldito snake vamos a aprender lo basico...
 */
pub fn main_variables() {
    let mut entrada: String = String::new();
    let mut entrada2: String = String::new();
    println!("¿Como te llamas?");
    leer_string_por_referencia(&mut entrada);
    println!("¿Cuales son tus aficiones?");
    entrada2 = leer_string_retornando_valor();
    print!("Tu nombre es: {} y tus aficiones son: {}", entrada.trim(), entrada2);
}

// paso de referencias
fn leer_string_por_referencia(result: &mut String) {
    io::stdin().read_line(result).expect("Error al leer");
}

// retorno de valores
fn leer_string_retornando_valor() -> String {
    let mut entrada: String = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada
}