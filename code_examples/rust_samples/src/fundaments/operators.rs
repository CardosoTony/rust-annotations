use crate::utils::terminal::selected_option;

pub fn arithmetics() {
    selected_option("Arithmetics");

    let x = 9;
    let y = 3;

    println!("x + y = {}", x + y);
    println!("x - y = {}", x - y);
    println!("x * y = {}", x * y);
    println!("x / y = {}", x / y);
    println!("x % y = {}\n", x % y);
}

pub fn relational() {
    selected_option("Relational");

    let x = 9;
    let y = 3;

    println!("x == y = {}", x == y);
    println!("x!= y = {}", x != y);
    println!("x > y = {}", x > y);
    println!("x < y = {}", x < y);
    println!("x >= y = {}", x >= y);
    println!("x <= y = {}\n", x <= y);
}

pub fn logical() {
    selected_option("Logical");

    let x = true;
    let y = false;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("x && y = {}", x && y);
    println!("x || y = {}", x || y);
    println!("!x = {}", !x);
    println!("!y = {}\n", !y);
}
