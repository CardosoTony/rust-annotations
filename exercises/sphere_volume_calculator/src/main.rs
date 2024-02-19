use std::{f64::consts::PI, io};

fn main() {
    println!("Enter the sphere radius [cm]: ");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Error reading");
    let radius: f64 = radius.trim().parse().expect("Error parsing");

    let volume = (4.0 * PI * radius.powi(3)) / 3.0;

    println!("The volume of the sphere is: {:.4} cm³.", volume);
}
