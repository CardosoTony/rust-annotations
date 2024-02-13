use std::io;

fn main() {
    println!("Enter the distance [km]:");
    let mut distance = String::new();
    io::stdin().read_line(&mut distance).unwrap();
    let distance: f64 = distance.trim().parse().unwrap();

    println!("Enter the time [min]:");
    let mut time = String::new();
    io::stdin().read_line(&mut time).unwrap();
    let time: f64 = time.trim().parse().unwrap();

    let speed = distance / (time / 60.0);

    println!("The average speed is {:.2} km/h", speed);
}
