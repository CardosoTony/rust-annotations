use std::io;

fn main() {
    println!("Enter the force [N]:");
    let mut force = String::new();
    io::stdin().read_line(&mut force).unwrap();
    let force: f64 = force.trim().parse().unwrap();

    println!("Enter the displacement [m]:");
    let mut displacement = String::new();
    io::stdin().read_line(&mut displacement).unwrap();
    let displacement: f64 = displacement.trim().parse().unwrap();

    let work = force * displacement;
    println!("Work is {} J", work);
}
