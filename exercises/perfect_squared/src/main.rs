use std::{io, io::Write};

fn print_perfect_square(value: u32) {
    let mut counter = 0;
    let mut number: u32 = 1;
    print!("Perfect squares: [");
    while counter < value {
        let perfect_square = number.pow(2);
        print!("{}", perfect_square);
        if counter < value - 1 {
            print!(", ");
        }
        counter += 1;
        number += 1;
    }
    println!("]");
}

fn main() {
    print!("Enter the value: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input!");
    let value: u32 = match input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Please enter a valid integer!");
            return;
        }
    };

    print_perfect_square(value);
}
