use std::{io, io::Write};

fn get_prompt(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.trim().parse().expect("Error parsing input")
}

fn for_mode() {
    let input_for = get_prompt("Enter a number: ");

    println!("The multiplication table for {} is: ", input_for);
    for i in 1..=10 {
        println!("{} x {} = {}", input_for, i, input_for * i);
    }
}

fn while_mode() {
    let input_while = get_prompt("Enter a number: ");

    println!("The multiplication table for {} is: ", input_while);
    let mut i = 1;
    while i <= 10 {
        println!("{} x {} = {}", input_while, i, input_while * i);
        i += 1;
    }
}

fn main() {
    for_mode();
    println!("{}", "-".repeat(10));
    while_mode();
}
