pub fn main_arrays_listas() {
    // arrays de toda la vida
    let mi_lista_de_numeros: [i32; 5] = [1,2,3,4,5];
    for numero in mi_lista_de_numeros.iter() {
        println!("Pintando elemento -> {}", numero)
    }
    let array_con_multiples_valores: [i32; 5] = [4096; 5];
    for numero in array_con_multiples_valores.iter() {
        println!("Pintando elemento -> {}", numero)
    }
    // listas (vectores)
    let mut lista = vec![1];
    // solo puedo llamar a push si he declarado la lista como mutable.
    lista.push(12354);
    lista.push(23456);
    match lista.get(1) {
        Some(valor) => println!("Valor obtenido correctamente, el valor es {}", valor),
        None => println!("No hay un valor en ese elemento.")
    }
    match lista.get(395839) {
        Some(valor) => println!("Valor obtenido correctamente, el valor es {}", valor),
        None => println!("No hay un valor en ese elemento.")
    }
    println!("Vector final: {:?}", lista);
}