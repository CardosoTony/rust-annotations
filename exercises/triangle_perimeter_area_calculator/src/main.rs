use std::io;

fn calculate_perimeter() -> f64 {
    println!("\nEnter the base value:");
    let base = input_float();

    println!("Enter the side 1 value:");
    let side1 = input_float();

    println!("Enter the side 2 value:");
    let side2 = input_float();

    base + side1 + side2
}

fn calculate_area() -> f64 {
    println!("\nEnter the base value:");
    let base = input_float();

    println!("Enter the height value:");
    let height = input_float();

    (base * height) / 2.0
}

fn input_float() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("\nError reading input");
    input.trim().parse().expect("\nError parsing input")
}

fn wait_for_continue() {
    println!("\nPress Enter to continue...");
    let mut _dummy = String::new();
    io::stdin()
        .read_line(&mut _dummy)
        .expect("\nFailed to read line");
}

fn main() {
    loop {
        println!("\nMenu:\n1 - Perimeter\n2 - Area\n0 - Exit\n");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid choice. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let perimeter = calculate_perimeter();
                println!("\nThe perimeter is: {}", perimeter);
                wait_for_continue();
            }
            2 => {
                let area = calculate_area();
                println!("\nThe area is: {}", area);
                wait_for_continue();
            }
            0 => {
                println!("\nExiting program!");
                break;
            }
            _ => println!("\nInvalid choice. Please enter a number."),
        }
    }
}
