use std::io;

fn main() {
    println!("Enter the first grade: ");
    let grade1 = read_grade();

    println!("Enter the second grade: ");
    let grade2 = read_grade();

    println!("Enter the third grade: ");
    let grade3 = read_grade();

    let average = (grade1 + grade2 + grade3) / 3.0;
    println!("Your average is: {}", average);
}

fn read_grade() -> f32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read!");
        match input.trim().parse() {
            Ok(grade) => return grade,
            Err(_) => println!("Invalid grade. Please enter again."),
        }
    }
}
