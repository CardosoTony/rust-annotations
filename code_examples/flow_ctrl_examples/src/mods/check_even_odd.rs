use std::{io, io::Write};

pub fn check_even_odd() {
    print!("Enter the number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading");
    let number: u32 = input.trim().parse().expect("Error parsing");

    if number % 2 == 0 {
        println!("The number is even!");
    } else {
        println!("The number is odd!");
    }
}
