use std::{io, io::Write};

fn get_numbers(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    input.trim().parse().expect("Error parsing input")
}

fn main() {
    let first_number = get_numbers("Enter the first number: ");
    let second_number = get_numbers("Enter the second number: ");
    let third_number = get_numbers("Enter the third number: ");

    let result = first_number + second_number + third_number;

    if result % 5 == 0 {
        println!("{} is divisible by 5", result);
    } else {
        println!("{} is not divisible by 5", result);
    }
}
