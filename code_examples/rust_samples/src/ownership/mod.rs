pub mod mechanism;
pub mod reference;
pub mod scope;
pub mod slice;

use crate::utils::terminal::{clear_screen, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let option = [
            "Moving Ownership #01",
            "Moving Ownership #02",
            "Scope - Basic",
            "Scope - Lifetime",
            "Scope - Move",
            "Scope - Clone",
            "Scope - Copy",
            "Reference - Immutable",
            "Reference - Mutable #01",
            "Reference - Mutable #02",
            "Reference - Mutable #03",
            "Reference - Dangling",
            "Slice",
        ];

        let selected_option = show_menu("Ownership", &option, true);

        clear_screen();

        match selected_option {
            1 => mechanism::example_a(),
            2 => mechanism::example_b(),
            3 => scope::basic_example(),
            4 => scope::lifetime_example(),
            5 => scope::move_example(),
            6 => scope::clone_example(),
            7 => scope::copy_example(),
            8 => reference::immutable_references(),
            9 => reference::mutable_references_01(),
            10 => reference::mutable_references_02(),
            11 => reference::mutable_references_03(),
            12 => reference::dangling_reference(),
            13 => slice::example(),
            _ => break,
        }

        wait_for_enter();
    }
}
