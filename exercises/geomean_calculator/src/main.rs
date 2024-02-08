use std::io;

fn main() {
    println!("Enter the first number: ");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Error reading");
    let num1: f64 = first_number.trim().parse().expect("Error parsing");

    println!("Enter the second number: ");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Error reading");
    let num2: f64 = second_number.trim().parse().expect("Error parsing");

    println!("Enter the third number: ");
    let mut third_number = String::new();
    io::stdin()
        .read_line(&mut third_number)
        .expect("Error reading");
    let num3: f64 = third_number.trim().parse().expect("Error parsing");

    println!("Geometric Mean: {}", mean(num1, num2, num3));
}

fn mean(x: f64, y: f64, z: f64) -> f64 {
    (x * y * z).powf(1.0 / 3.0)
}
