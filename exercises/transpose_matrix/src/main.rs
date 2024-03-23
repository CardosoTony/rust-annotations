use rand::Rng;
fn main() {
    const N: usize = 4;
    let mut matrix: [[i32; N]; N] = [[0; N]; N];

    let mut rng = rand::thread_rng();

    for i in 0..N {
        for j in 0..N {
            matrix[i][j] = rng.gen_range(0..=100);
        }
    }

    println!("Original matrix:");

    for i in 0..N {
        for j in 0..N {
            print!("{:4} ", matrix[i][j]);
        }
        println!();
    }

    let mut transposed_matrix: [[i32; N]; N] = [[0; N]; N];

    for i in 0..N {
        for j in 0..N {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }

    println!("\nTransposed matrix:");

    for i in 0..N {
        for j in 0..N {
            print!("{:4} ", transposed_matrix[i][j]);
        }
        println!();
    }
}
