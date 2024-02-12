use std::io;

fn main() {
    println!("Enter the width: ");
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect("Error reading");
    let width: f64 = width.trim().parse().expect("Error parsing");

    println!("Enter the length: ");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Error reading");
    let length: f64 = length.trim().parse().expect("Error parsing");

    let perimeter = 2.0 * (width + length);
    let area = width * length;

    println!("Perimeter: {}", perimeter);
    println!("Area: {}", area);
}
