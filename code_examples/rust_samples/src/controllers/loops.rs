use crate::utils::terminal::selected_option;

pub fn example_range() {
    selected_option("Range");

    for a in 1..5 {
        println!("a = {}", a);
    }
    println!();

    for b in 1..=5 {
        println!("b = {}", b);
    }
    println!();

    for c in (1..5).rev() {
        println!("c = {}", c);
    }
    println!();

    for d in (5..=15).step_by(2) {
        println!("d = {}", d);
    }
    println!();
}

pub fn example_array() {
    selected_option("Array");

    let array = [1, 2, 3, 4, 5];

    for i in 0..array.len() {
        println!("array[{}] = {}", i, array[i]);
    }
    println!();

    for value in array {
        println!("value = {}", value);
    }
    println!();

    for (index, value) in array.iter().enumerate() {
        println!("index = [{}], value = {}", index, value);
    }
    println!();
}

pub fn example_while() {
    selected_option("While");

    let mut a = 1;

    while a <= 10 {
        println!("a = {}", a);
        a += 1;
    }
    println!();
}

pub fn example_loop() {
    selected_option("Loop");

    let mut a = 1;

    loop {
        println!("a = {}", a);
        a += 1;

        if a > 10 {
            break;
        }
    }
    println!();
}
