use std::{io, io::Write};

fn get_input(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Failed to parse")
}

fn main() {
    let n1 = get_input("Enter the first number: ");
    let n2 = get_input("Enter the second number: ");
    println!("Number between {} and {}", n1, n2);

    if n1 <= n2 {
        print!("[");
        for i in n1..=n2 {
            if i != n2 {
                print!("{}, ", i);
            } else {
                print!("{}", i);
            }
        }
    } else {
        for i in n2..=n1 {
            if i != n1 {
                print!("{}, ", i);
            } else {
                print!("{}", i);
            }
        }
    }
    println!("]");
}
