use std::{io, io::Write};

fn main() {
    let mut sum_positive = 0;
    loop {
        print!("Enter a number (enter a negative number to end): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        if number < 0 {
            break;
        } else {
            sum_positive += number;
        }
    }

    println!("Sum of positive numbers: {}", sum_positive);
}
