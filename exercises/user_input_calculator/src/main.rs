use std::io;
use user_input_calculator::{divide, multiply, subtract, sum};

fn main() {
    println!("Type the first number: ");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Error reading number!");
    println!("Type the second number: ");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Error reading number!");

    let x: f64 = first_number.trim().parse().expect("Failed to parse");
    let y: f64 = second_number.trim().parse().expect("Failed to parse");
    sum(x, y);
    subtract(x, y);
    multiply(x, y);
    divide(x, y);
}
