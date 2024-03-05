use std::{io, io::Write};

fn calculate_series_resistance(resistor1: f32, resistor2: f32) -> f32 {
    resistor1 + resistor2
}

fn calculate_parallel_resistance(resistor1: f32, resistor2: f32) -> f32 {
    (resistor1 * resistor2) / (resistor1 + resistor2)
}

fn get_resistor_value(prompt: &str) -> f32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    input.trim().parse().expect("Please enter a valid number.")
}

fn main() {
    let resistor1_value = get_resistor_value("Enter the resistance value of resistor 1: ");
    let resistor2_value = get_resistor_value("Enter the resistance value of resistor 2: ");

    let series_resistance = calculate_series_resistance(resistor1_value, resistor2_value);
    let parallel_resistance = calculate_parallel_resistance(resistor1_value, resistor2_value);

    println!(
        "The resulting resistance when connected in series is: {:.2} ohms.",
        series_resistance
    );
    println!(
        "The resulting resistance when connected in parallel is: {:.2} ohms.",
        parallel_resistance
    );
}
