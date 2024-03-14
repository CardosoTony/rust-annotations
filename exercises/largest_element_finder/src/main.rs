use std::{io, io::Write};

fn main() {
    print!("Enter the numbers separated by space: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|num_str| num_str.parse().ok())
        .collect();

    if numbers.is_empty() {
        println!("No valid numbers was entered.");
    } else {
        let mut max_number = numbers[0];
        for &number in &numbers {
            if number > max_number {
                max_number = number;
            }
        }
        println!("The largest number is {}", max_number);
    }
}
