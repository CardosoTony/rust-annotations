fn main() {
    immutable_variable();
    mutable_variable();
    handle_constants();
    shadowing();
}

fn immutable_variable() {
    // Immutable variables
    let x: i32 = 6; // Declare a variable 'x' with type 'i32' and assign the value '6'
    let y = 23.2; // The compiler infers type 'f64' for variable 'y' based on the value '23.2'
    let message: &str = "Hello, world!"; // String type

    // Print the values to check if they are correct
    println!("Value of x: {}", x);
    println!("Value of y: {}", y);
    println!("Message: {}", message);
}

fn mutable_variable() {
    // Mutable variables
    let mut counter: u8 = 32; // Declare a mutable variable 'counter' with type 'u8' and assign the value '32'
    println!("Value of counter: {}", counter);

    counter += 3; // Increment the value of 'counter' by 3
    println!("Value of counter: {}", counter);

    counter = 18; // It is possible to change the value of 'counter'
    println!("Value of counter: {}", counter);
}

fn handle_constants() {
    // Constants
    const PI: f64 = 3.14159; // Declare a constant 'PI' with type 'f64' and assign the value '3.14159'
    const LIMIT_RANGE: u8 = 15; // Declare a constant 'LIMIT_RANGE' with type 'u8'

    // Print constant values
    println!("Constant PI: {}", PI);
    println!("Constant LIMIT_RANGE: {}", LIMIT_RANGE);
}

fn shadowing() {
    // Shadowing
    let x = 8;
    println!("Value of x: {}", x);

    let mut x = x + 10;
    println!("Value of x: {}", x);

    x = 32;
    let y = x + 14;
    println!("Value of x: {}", y);

    // NOTE: Allows reusing the name of a variable when declaring a new variable with the same name. In this case, the new variable overshadows the previous one but can have a different type or be mutable."
}
