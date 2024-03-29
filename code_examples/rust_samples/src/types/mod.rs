pub mod basics;
pub mod customs;
pub mod sequences;

use crate::utils::terminal::{clear_screen, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let option = [
            "Basics",
            "Sequences",
            "Custom - Structs",
            "Custom - Enums",
            "Custom - Traits",
        ];

        let selected_option = show_menu("Types", &option, true);

        clear_screen();

        match selected_option {
            1 => basics::example(),
            2 => sequences::example(),
            3 => customs::structs(),
            4 => customs::enums(),
            5 => customs::trais(),
            _ => break,
        }

        wait_for_enter();
    }
}
