use std::io;

fn main() {
    println!("Enter the radius [mm]: ");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Error reading");
    let radius: f64 = radius.trim().parse().expect("Error parsing");

    let circumference = 2.0 * std::f64::consts::PI * radius;

    println!("The circumference is {:.2} mm", circumference);
}
