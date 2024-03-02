use std::{io, io::Write};

fn main() {
    print!("Enter the number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading");
    let number: i32 = input.trim().parse().expect("Error parsing");

    let mut primes_below_input = Vec::new();

    for numbers in 2..number {
        if is_prime(numbers) {
            primes_below_input.push(numbers);
        }
    }

    if primes_below_input.is_empty() {
        println!(
            "{} is neither prime nor composite as it has only one divisor, itself.",
            number
        );
    } else {
        println!(
            "There are {} prime numbers lower than {} in total: ",
            primes_below_input.len(),
            number
        );
        print_primes(primes_below_input);
    }
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn print_primes(primes: Vec<i32>) {
    for (index, prime) in primes.iter().enumerate() {
        if index > 0 {
            print!(", ");
        }
        print!("{}", prime);
    }
    println!();
}
