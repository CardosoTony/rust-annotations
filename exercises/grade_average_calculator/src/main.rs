use std::io;

fn main() {
    grade_average();
}

fn grade_average() {
    println!("Enter the first grade: ");
    let mut first_grade = String::new();
    io::stdin()
        .read_line(&mut first_grade)
        .expect("Error reading first grade!");

    println!("Enter the second grade: ");
    let mut second_grade = String::new();
    io::stdin()
        .read_line(&mut second_grade)
        .expect("Error reading second grade!");

    println!("Enter the third grade: ");
    let mut third_grade = String::new();
    io::stdin()
        .read_line(&mut third_grade)
        .expect("Error reading third grade!");

    let first_grade: f32 = first_grade
        .trim()
        .parse()
        .expect("Error parsing first grade!");
    let second_grade: f32 = second_grade
        .trim()
        .parse()
        .expect("Error parsing second grade!");
    let third_grade: f32 = third_grade
        .trim()
        .parse()
        .expect("Error parsing third grade!");

    let average = (first_grade + second_grade + third_grade) / 3.0;
    let formatted_average = format!("{:.2}", average);
    println!("Your average is: {}", formatted_average);
}
