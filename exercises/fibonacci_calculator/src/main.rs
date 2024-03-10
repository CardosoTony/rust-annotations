use std::{io, io::Write};

fn get_input(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            get_input(prompt)
        }
    };

    return n;
}

fn sequence_until_number(n: u32) {
    let mut current = 0;
    let mut next = 1;

    print!("[");
    while current <= n {
        print!("{}", current);
        current = next;
        next = current + next;
        if current <= n {
            print!(", ");
        }
    }
    println!("]");
}

fn sequence_first_n_number(n: u32) {
    let mut f_0 = 0;
    let mut f_1 = 1;

    print!("[");
    for i in 0..n {
        print!("{}", f_0);
        if i < n - 1 {
            print!(", ");
        }
        let f_next = f_0 + f_1;
        f_0 = f_1;
        f_1 = f_next;
    }
    println!("]");
}

fn main() {
    let until_number =
        get_input("Enter a number to display the Fibonacci sequence up to that number: ");
    sequence_until_number(until_number);

    let first_n_number =
        get_input("Enter a number to display the first 'n' numbers of the Fibonacci sequence: ");
    sequence_first_n_number(first_n_number);
}
