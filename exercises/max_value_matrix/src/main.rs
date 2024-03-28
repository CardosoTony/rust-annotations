use std::{io, io::Write};

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn read_matrix_value(i: usize, j: usize) -> f32 {
    loop {
        print!("Enter the value at position [{}, {}]: ", i, j);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input.");
        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input! Please enter a valid number."),
        }
    }
}

fn read_matrix(size: usize) -> Vec<Vec<f32>> {
    let mut matrix = vec![vec![0.0; size]; size];
    println!("\nEnter the values for the {}x{} matrix\n", size, size);

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = read_matrix_value(i, j);
        }
    }

    matrix
}

fn find_max_value_and_position(matrix: &Vec<Vec<f32>>) -> (f32, (usize, usize)) {
    let mut max_value = matrix[0][0];
    let mut max_position = (0, 0);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] > max_value {
                max_value = matrix[i][j];
                max_position = (i, j);
            }
        }
    }

    (max_value, max_position)
}

fn main() {
    clear_screen();

    print!("\nEnter the size of the matrix: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.");
    let size: usize = input.trim().parse().expect("Enter a valid number!");

    let matrix = read_matrix(size);

    let (max_value, max_position) = find_max_value_and_position(&matrix);

    println!(
        "\nThe maximum value is {} at position [{}, {}]\n",
        max_value, max_position.0, max_position.1
    );
}
