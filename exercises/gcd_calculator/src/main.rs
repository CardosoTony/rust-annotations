use std::{io, io::Write};

// Input loop, type "end" to finish
fn get_input(prompt: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        if input.trim() == "end" {
            break;
        }

        match input.trim().parse() {
            Ok(num) => numbers.push(num),
            Err(_) => {
                println!("Invalid input.");
            }
        }
    }
    numbers
}

// Greatest Common Divisor
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    println!("GCD Calculator - Greatest Common Divisor");

    let numbers = get_input("Enter a value (type 'end' to finish): ");

    if numbers.len() < 2 {
        println!("At least two values are required.");
        return;
    }

    let mut result = numbers[0];

    for &number in &numbers[1..] {
        result = gcd(result, number)
    }

    print!("The GCD between [");
    for (index, &number) in numbers.iter().enumerate() {
        if index == numbers.len() - 1 {
            print!("{}", number);
        } else {
            print!("{}, ", number);
        }
    }
    println!("] is: {}", result);
}
