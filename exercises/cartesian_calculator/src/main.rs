use std::io;

fn main() {
    println!("Enter the first coordinate [x1, y1]");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    println!("Enter the second coordinate [x2, y2]");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let coords1: Vec<f64> = input1
        .trim()
        .split(',')
        .map(|substring| substring.trim().parse::<f64>())
        .filter_map(Result::ok)
        .collect();

    let coords2: Vec<f64> = input2
        .trim()
        .split(',')
        .map(|substring| substring.trim().parse::<f64>())
        .filter_map(Result::ok)
        .collect();

    if coords1.len() != 2 || coords2.len() != 2 {
        println!("Invalid input.\nPlease enter two number for each coordinate.");
        return;
    }

    let distance =
        ((coords2[0] - coords1[0]).powf(2.0) + (coords2[1] - coords1[1]).powf(2.0)).sqrt();

    println!("The distance between the two points is: {:.4}", distance);
}
