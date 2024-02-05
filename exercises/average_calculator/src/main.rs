use std::io;

fn main() {
    two_number_average();
}

fn two_number_average() {
    println!("Type the first number: ");

    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Error reading number!");

    println!("Type the second number: ");

    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Error reading number!");

    let first_number: f32 = first_number.trim().parse().expect("Error parsing number");
    let second_number: f32 = second_number.trim().parse().expect("Error parsing number");

    let average = (first_number + second_number) / 2.0;

    println!(
        "The average of {} and {} is: {}",
        first_number, second_number, average
    );
}
