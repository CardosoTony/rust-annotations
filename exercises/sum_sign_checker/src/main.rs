use std::{io, io::Write};

fn get_numbers(prompt: &str) -> f32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.trim().parse().expect("Error parsing input")
}

fn sign_checker(n1: f32, n2: f32, n3: f32) {
    let sum = n1 + n2 + n3;

    if sum > 0.0 {
        println!("Sum is positive");
    } else if sum < 0.0 {
        println!("Sum is negative");
    } else {
        println!("Sum is zero");
    }
}

fn main() {
    let first_number = get_numbers("Enter the first number: ");
    let second_number = get_numbers("Enter the second number: ");
    let third_number = get_numbers("Enter the third number: ");

    sign_checker(first_number, second_number, third_number);
}
