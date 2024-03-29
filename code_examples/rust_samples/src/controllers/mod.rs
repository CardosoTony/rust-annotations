pub mod conditionals;
pub mod loops;

use crate::utils::terminal::{clear_screen, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let option = ["Conditionals", "For - Range", "For Array", "While", "Loops"];

        let selected_option = show_menu("Controllers", &option, true);

        clear_screen();

        match selected_option {
            1 => conditionals::example(),
            2 => loops::example_range(),
            3 => loops::example_array(),
            4 => loops::example_while(),
            5 => loops::example_loop(),
            _ => break,
        }

        wait_for_enter();
    }
}
