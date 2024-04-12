mod main_test;

use std::io::{self, Write};

pub fn get_matrix_size() -> usize {
    println!("\n=== Square Matrix ===");
    println!("\n== Type 0 to exit. ==");

    loop {
        let input_size = prompt_input("\nEnter the size of the matrix: ");

        match input_size.trim().parse() {
            Ok(value) if value == 0 => {
                println!("\n== Exiting program ==");
                std::process::exit(0);
            }
            Ok(value) if value >= 2 => return value,
            Ok(_) => println!("\nInvalid input! Matrix size must be greater than or equal to 2."),
            Err(_) => println!("\nInvalid input! Please enter a valid number."),
        };
    }
}

pub fn read_matrix(size: usize) -> Vec<Vec<f32>> {
    let mut matrix = vec![vec![0.0; size]; size];

    println!("\nEnter the values for the {}x{} matrix.", size, size);

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = get_matrix_value(i, j);
        }
    }

    matrix
}

pub fn average_of_even_values(
    size: usize,
    matrix: &Vec<Vec<f32>>,
    return_result: bool,
) -> Option<f32> {
    let mut sum_even = 0.0;
    let mut count_even = 0;

    for i in 0..size {
        for j in 0..size {
            if (i + j) % 2 == 0 {
                sum_even += matrix[i][j];
                count_even += 1;
            }
        }
    }

    if count_even > 0 {
        let result = sum_even / count_even as f32;
        if return_result {
            Some(result)
        } else {
            println!("\nAverage of even values: {:.2}", result);
            None
        }
    } else {
        None
    }
}

fn clear_console_screen() {
    print!("{esc}c", esc = 27 as char)
}

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.");

    input
}

fn get_matrix_value(i: usize, j: usize) -> f32 {
    loop {
        let input_value = prompt_input(&format!("\nEnter the value at position [{}, {}]: ", i, j));

        match input_value.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("\nInvalid input! Please enter a valid numerical value."),
        }
    }
}

fn main() {
    clear_console_screen();

    let matrix_size = get_matrix_size();

    let matrix = read_matrix(matrix_size);

    if let Some(result) = average_of_even_values(matrix_size, &matrix, true) {
        println!("\nAverage of even values: {:.2}", result);
    } else {
        println!("\nNo even values found.");
    }
}
