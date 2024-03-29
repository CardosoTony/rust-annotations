use std::process::exit;

pub mod controllers;
pub mod functions;
pub mod fundaments;
pub mod ownership;
pub mod types;
pub mod utils;

use utils::terminal::{clear_screen, show_menu};

fn main() {
    loop {
        let options = [
            "Fundaments",
            "Types",
            "Controllers",
            "Functions",
            "Ownership",
        ];

        let selected_options = show_menu("Main", &options, true);

        clear_screen();

        match selected_options {
            1 => fundaments::execute(),
            2 => types::execute(),
            3 => controllers::execute(),
            4 => functions::execute(),
            5 => ownership::execute(),
            _ => exit(0),
        }
    }
}
