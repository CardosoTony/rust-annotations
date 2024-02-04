fn main() {
    println_method();
    print_method();
    debug_method();
    eprintln_method();
}

fn println_method() {
    let name = "Rust";
    let year = 2024;
    println!("Learning {} in {}", name, year);
}

fn print_method() {
    let name = "Rust";
    let year = 2024;
    print!("Hello! ");
    print!("I'm learning {} in {}\n", name, year);
}

fn debug_method() {
    let x: i8 = 5;
    let y: i8 = 15;

    dbg!(x + y);
}

fn eprintln_method() {
    let error = "Your error message.";

    eprintln!("Error: {}", error);
}
