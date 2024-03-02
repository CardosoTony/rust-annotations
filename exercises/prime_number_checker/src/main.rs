use std::{io, io::Write};

fn main() {
    print!("Enter the number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading");
    let number: i32 = input.trim().parse().expect("Error parsing");

    if number == 1 {
        println!(
            "{} is neither prime nor composite as it has only one divisor, itself.",
            number
        );
    } else if is_prime(number) {
        println!("{} is a prime number", number);
    } else {
        println!("{} is not a prime number", number);
    }
}

fn is_prime(number: i32) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}
