#[path = "learning_basics/variables_and_functions.rs"]
mod variables_and_functions;

#[path = "learning_basics/structures.rs"]
mod structures;

#[path = "learning_basics/arrays_listas.rs"]
mod arrays_listas;

#[path = "snake/snakegame_ggez.rs"]
mod snake_ggez;

use ggez::GameResult;

fn main() -> GameResult {
    // conceptos basicos de rust antes de nada...
    // variables_and_functions::main_variables();
    //structures::main_structures();
    // arrays_listas::main_arrays_listas();
    snake_ggez::main()
}