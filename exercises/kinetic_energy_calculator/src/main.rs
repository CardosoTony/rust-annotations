use std::io;

fn main() {
    println!("Enter the mass [kg]:");
    let mut mass = String::new();
    io::stdin().read_line(&mut mass).unwrap();
    let mass: f64 = mass.trim().parse().unwrap();

    println!("Enter the velocity [m/s]:");
    let mut velocity = String::new();
    io::stdin().read_line(&mut velocity).unwrap();
    let velocity: f64 = velocity.trim().parse().unwrap();

    let kinetic_energy = mass * velocity.powf(2.0) / 2.0;
    println!("The kinetic energy is: {} J", kinetic_energy);
}
