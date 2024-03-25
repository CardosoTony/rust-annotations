use std::{io, io::Write};

const N: usize = 2;

fn input_matrix() -> [[i32; N]; N] {
    let mut matrix: [[i32; N]; N] = [[0; N]; N];

    for i in 0..N {
        for j in 0..N {
            loop {
                print!("Enter the value at position [{}, {}]: ", i, j);
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading input.");
                match input.trim().parse() {
                    Ok(value) => {
                        matrix[i][j] = value;
                        break;
                    }
                    Err(_) => println!("Enter a valid number."),
                }
            }
        }
    }
    matrix
}

fn sum_matrix(matrix_a: [[i32; N]; N], matrix_b: [[i32; N]; N]) -> [[i32; N]; N] {
    let mut sum_matrix: [[i32; N]; N] = [[0; N]; N];

    for i in 0..N {
        for j in 0..N {
            sum_matrix[i][j] = matrix_a[i][j] + matrix_b[i][j];
        }
    }
    sum_matrix
}
fn main() {
    println!("\nEnter the values of the first 2x2 matrix");
    let matrix_a = input_matrix();

    println!("\nEnter the values of the second 2x2 matrix");
    let matrix_b = input_matrix();

    let sum_matrix = sum_matrix(matrix_a, matrix_b);

    println!("\nResult of the sum of the two matrices:");
    for i in 0..N {
        for j in 0..N {
            print!("{:4}", sum_matrix[i][j]);
        }
        println!();
    }
}
