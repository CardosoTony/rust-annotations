use std::{io, io::Write};

pub fn check_positive_negative() {
    print!("Enter the number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let number: i32 = input.trim().parse().expect("Error parsing number");

    if number > 0 {
        println!("The number is positive");
    } else if number < 0 {
        println!("The number is negative");
    } else {
        println!("The number is zero");
    }
}
