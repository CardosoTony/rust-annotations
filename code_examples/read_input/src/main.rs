use std::io;

fn main() {
    str_input();
    number_input();
}

fn str_input() {
    println!("Type your name: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Hello, {}!", input.trim());
}

fn number_input() {
    println!("Type a number: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: f64 = input.trim().parse().expect("Failed to parse number");

    println!("Your number is {}", number);
}
