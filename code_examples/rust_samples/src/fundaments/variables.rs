use crate::utils::terminal::selected_option;

pub fn immutables() {
    selected_option("Immutables");

    let x = 5;
    // x = 10;
    println!("x = {}\n", x);
}

pub fn mutable() {
    selected_option("Mutable");

    let mut x = 5;
    let y = x;
    println!("x, y = {}, {}", x, y);

    x = 10;
    println!("x, y = {}, {}\n", x, y);
}

pub fn constants() {
    selected_option("Constants");

    const Z: i32 = 13;
    println!("Z = {}\n", Z);
}

pub fn shadowing() {
    selected_option("Shadowing");

    let a = 19;
    println!("a = {}", a);

    let a = "text";
    println!("a = {}", a);

    let a = [2, 3, 4, 5, 6];
    println!("a = {:?}\n", a);
}
