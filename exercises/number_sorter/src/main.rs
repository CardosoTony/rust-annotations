use std::{io, io::Write};

fn main() {
    let mut numbers = Vec::new();

    for i in 0..3 {
        print!("Enter number {}: ", i + 1);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        let number: i32 = input.trim().parse().expect("Error parsing");
        numbers.push(number);
    }
    numbers.sort();

    println!("Numbers in ascending order: {:?}", numbers);
}
