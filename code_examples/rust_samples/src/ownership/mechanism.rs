use crate::utils::terminal::selected_option;

pub fn example_a() {
    selected_option("Ownership #01");

    let name = String::from("Brian");
    show_name(name);

    // println!("{}", name); // error message

    let age = 25;
    show_age(age);

    println!("{}\n", age);
}

// Ownership is moved to the argument taking possession of the value
fn show_name(name: String) {
    println!("Name: {}", name);
} // name is discarded (drop is called)

// Receives a copy of the value (doesn't take ownership)
fn show_age(age: u8) {
    println!("Age: {}", age);
}

pub fn example_b() {
    selected_option("Ownership #02");

    let name = new_name();
    println!("{}", name);

    let (name, size) = calculate_size(name);
    println!("Name: {}, Size: {}\n", name, size);
}

fn new_name() -> String {
    let name = String::from("Sophia");
    name // ownership is transferred to the calling function
}

fn calculate_size(name: String) -> (String, usize) {
    let size = name.len();
    (name, size) // Returns ownership of the value to the calling function
}
