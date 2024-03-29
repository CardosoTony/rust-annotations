pub mod first;
pub mod operators;
pub mod variables;

use crate::utils::terminal::{clear_screen, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let option = [
            "First Example",
            "Variables - Immutables",
            "Variables - Mutable",
            "Variables - Constants",
            "Variables - Shadowing",
            "Operators - Arithmetics",
            "Operators - Relational",
            "Operators - Logical",
        ];

        let selected_option = show_menu("Fundaments", &option, true);

        clear_screen();

        match selected_option {
            1 => first::example(),
            2 => variables::immutables(),
            3 => variables::mutable(),
            4 => variables::constants(),
            5 => variables::shadowing(),
            6 => operators::arithmetics(),
            7 => operators::relational(),
            8 => operators::logical(),
            _ => break,
        }

        wait_for_enter();
    }
}
