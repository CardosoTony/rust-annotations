use std::io;

fn main() {
    println!("Enter the radius [mm]:");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Error reading");
    let radius: f64 = radius.trim().parse().expect("Error parsing");

    let area = std::f64::consts::PI * radius.powf(2.0);
    println!("The area of the circle is: {} mm²", area);

    println!("\nPress Enter to exit...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).expect("Error reading");
}
