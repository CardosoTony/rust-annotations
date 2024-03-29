use std::{io, io::Write};

use rpassword::prompt_password;

pub fn show_menu(title: &str, options: &[&str], exit: bool) -> u32 {
    clear_screen();

    let full_title = String::from("Rust Menu | ") + title;

    println!("{}", full_title);
    println!("{}", String::from("=").repeat(full_title.len()));

    show_options(options);

    println!("{}", if exit { "0 - Exit" } else { "0 - Back" });

    print!("\nSelect an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let chosen_option: Result<u32, _> = input.trim().parse();

    match chosen_option {
        Ok(chosen_option) => chosen_option,
        _ => 0,
    }
}

fn show_options(options: &[&str]) {
    for (i, option) in options.iter().enumerate() {
        println!("{} - {}", i + 1, option);
    }
}

pub fn wait_for_enter() {
    prompt_password("Press ENTER to continue...").unwrap();
}

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

pub fn selected_option(option: &str) {
    println!("{}", option);
    println!("{}", "=".repeat(option.len()));
}
