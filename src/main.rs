//Using the tui_functions.rs file and calling all functions within into the main.rs file's namespace
mod tui_functions;
use crate::tui_functions::*;

//Main application and the order that the functions are called
fn main() {
    menu();
}