use std::{io, io::Write};

fn main() {
    const N: usize = 3;
    let mut matrix: [[i32; N]; N] = [[0; N]; N];

    println!("\nEnter the value of the 3x3 matrix.");

    for i in 0..N {
        for j in 0..N {
            loop {
                print!("\nEnter the value of position [{}, {}]: ", i, j);
                io::stdout().flush().unwrap();

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("\nError reading input!");

                match input.trim().parse() {
                    Ok(value) => {
                        matrix[i][j] = value;
                        break;
                    }
                    Err(_) => {
                        println!("\nEnter a valid value!");
                        continue;
                    }
                }
            }
        }
    }

    let mut sum = 0;

    for i in 0..N {
        sum += matrix[i][i];
    }

    println!("\nThe sum of the matrix is: {}", sum);
}
