use rand::Rng;

fn create_matrix(matrix_size: usize) -> (usize, Vec<Vec<i32>>) {
    let mut matrix = vec![vec![0; matrix_size]; matrix_size];
    let mut rng = rand::thread_rng();

    for i in 0..matrix_size {
        for j in 0..matrix_size {
            matrix[i][j] = rng.gen_range(0..=100);
        }
    }

    // Print the matrix
    // Mode 1:
    // for row in matrix.iter() {
    //     println!("{:?}", row);
    // }

    // Mode 2:
    for i in 0..matrix_size {
        print!("|");
        for j in 0..matrix_size {
            print!(" {:<3} ", matrix[i][j]);
        }
        println!("|");
    }

    (matrix_size, matrix)
}

fn sum_rows(matrix_size: usize, matrix: &Vec<Vec<i32>>) {
    println!();
    for i in 0..matrix_size {
        let mut row_sum: i32 = 0;
        for j in 0..matrix_size {
            row_sum += matrix[i][j];
        }
        println!("Sum of row {}: {}", i + 1, row_sum);
    }
}

fn sum_columns(matrix_size: usize, matrix: &Vec<Vec<i32>>) {
    println!();
    for j in 0..matrix_size {
        let mut column_sum: i32 = 0;
        for i in 0..matrix_size {
            column_sum += matrix[i][j];
        }
        println!("Sum of column {}: {}", j + 1, column_sum);
    }
}

fn main() {
    // MODE 1:
    /*
    const SIZE: usize = 4;

    let mut matrix: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut rng = rand::thread_rng();

    for i in 0..SIZE {
        for j in 0..SIZE {
            matrix[i][j] = rng.gen_range(0..=100);
        }
    }

    // Print the matrix
    for i in 0..SIZE {
        print!("|");
        for j in 0..SIZE {
            print!(" {:<3} ", matrix[i][j]);
        }
        println!("|");
    }

    // Sum of rows
    for i in 0..SIZE {
        let mut row_sum = 0;
        for j in 0..SIZE {
            row_sum += matrix[i][j];
        }
        println!("Sum of row {}: {}", i + 1, row_sum);
    }

    // Sum of columns
    for j in 0..SIZE {
        let mut column_sum = 0;
        for i in 0..SIZE {
            column_sum += matrix[i][j];
        }
        println!("Sum of column {}: {}", j + 1, column_sum);
    }
    */

    // MODE 2:
    let (matrix_size, matrix) = create_matrix(4);
    sum_rows(matrix_size, &matrix);
    sum_columns(matrix_size, &matrix);
}
