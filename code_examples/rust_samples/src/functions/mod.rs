pub mod functions;
pub mod lambda;

use crate::utils::terminal::{clear_screen, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let option = ["Functions", "Map", "Filter"];

        let selected_option = show_menu("Functions", &option, true);

        clear_screen();

        match selected_option {
            1 => functions::functions(),
            2 => lambda::map_example(),
            3 => lambda::filter_example(),
            _ => break,
        }

        wait_for_enter();
    }
}
