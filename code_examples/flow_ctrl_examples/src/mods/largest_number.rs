use std::{cmp, io, io::Write};

pub fn find_largest_number() {
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Error reading");
    let first_number: u32 = first_number.trim().parse().expect("Error parsing");

    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Error reading");
    let second_number: u32 = second_number.trim().parse().expect("Error parsing");

    print!("Enter the third number: ");
    io::stdout().flush().unwrap();
    let mut third_number = String::new();
    io::stdin()
        .read_line(&mut third_number)
        .expect("Error reading");
    let third_number: u32 = third_number.trim().parse().expect("Error parsing");

    let largest = if first_number > second_number && first_number > third_number {
        first_number
    } else if second_number > first_number && second_number > third_number {
        second_number
    } else {
        third_number
    };

    println!("The largest number is: {}", largest);
}

fn get_number_for_alternative(prompt: &str) -> u32 {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading");

    input.trim().parse().expect("Error parsing")
}

pub fn find_largest_number_alternative() {
    let first_number = get_number_for_alternative("Enter the first number: ");
    let second_number = get_number_for_alternative("Enter the second number: ");
    let third_number = get_number_for_alternative("Enter the third number: ");

    let largest = cmp::max(cmp::max(first_number, second_number), third_number);

    println!("The largest number is: {}", largest);
}
