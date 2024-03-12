use std::{io, io::Write};

fn main() {
    println!("Find the minimum and maximum values!");

    let mut min_value = i32::MAX;
    let mut max_value = i32::MIN;

    loop {
        print!("Enter a number (enter 0 to end): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        if number == 0 {
            break;
        } else {
            if number < min_value {
                min_value = number;
            }
            if number > max_value {
                max_value = number;
            }
        }
    }

    if max_value != i32::MIN && min_value != i32::MAX {
        println!(
            "The minimum value is {} and the maximum value is {}.",
            min_value, max_value
        );
    } else {
        println!("No values entered.");
    }
}
