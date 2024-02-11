use std::io;

fn input_prompt(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.trim().parse().expect("Error parsing input")
}

fn format_coefficient(coefficient: f32) -> String {
    if coefficient >= 0.0 {
        format!("+ {}", coefficient)
    } else {
        format!("- {}", -coefficient)
    }
}

fn main() {
    let a = input_prompt("Enter the value a: ");
    let b = input_prompt("Enter the value b: ");
    let c = input_prompt("Enter the value c: ");

    let delta = f32::powf(b, 2.0) - (4.0 * a * c);

    if delta < 0.0 {
        let function_formatted = format!(
            "{}x² {}x {} = 0",
            a,
            format_coefficient(b),
            format_coefficient(c)
        );

        println!("\nf(x) = {}\n", function_formatted);

        println!("Delta = {}\n", delta);

        println!("Delta is negative. The equation has no real roots.");
    } else {
        let function_formatted = format!(
            "{}x² {}x {} = 0",
            a,
            format_coefficient(b),
            format_coefficient(c)
        );

        let x1 = (-b + delta.sqrt()) / (2.0 * a);
        let x2 = (-b - delta.sqrt()) / (2.0 * a);

        println!("\nf(x) = {}\n", function_formatted);

        println!("Delta = {}\n", delta);

        println!("x' => {}", x1);
        println!("x\" => {}", x2);
    }
}
