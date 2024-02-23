use std::{io, io::Write};

fn get_grade(prompt: &str) -> f32 {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    input.trim().parse().expect("Failed to parse")
}

fn main() {
    let first_grade = get_grade("Enter the first grade: ");
    let second_grade = get_grade("Enter the second grade: ");
    let third_grade = get_grade("Enter the third grade: ");

    let mean_grade = (first_grade + second_grade + third_grade) / 3.0;

    if mean_grade >= 6.0 {
        println!("Mean: {:.2} | The student passed!", mean_grade);
    } else {
        println!("Mean: {:.2} | The student failed!", mean_grade);
    }
}
