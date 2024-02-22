use std::{io, io::Write};

pub fn age_verification() {
    print!("Enter your age: ");
    io::stdout().flush().unwrap();

    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Error reading");
    let age: u32 = age.trim().parse().expect("Error parsing");

    if age < 18 {
        println!("You are not old enough!");
    } else {
        println!("You are old enough!");
    }
}
