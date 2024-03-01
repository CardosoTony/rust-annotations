use std::{io, io::Write};

fn main() {
    print!("Enter the number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading");
    let input_number: i32 = input.trim().parse().expect("Error parsing");

    let mut sum = 0;
    for number in 1..=input_number {
        sum += number;
    }

    println!(
        "The sum of the numbers from 1 to {} is: {}",
        input_number, sum
    );
}
