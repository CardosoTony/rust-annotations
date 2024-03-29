use crate::utils::terminal::selected_option;

pub fn example() {
    selected_option("Basics");

    let boolean: bool = true;
    println!("Boolean: {}", boolean);

    let character: char = 'A';
    println!("Character: {}", character);

    let string: &str = "Hello, world!";
    println!("String: {}", string);

    let mut string: String = String::from("Hello, ");
    string.push_str("world!");
    println!("String: {}", string);

    // i8, i16, i32, i64, i128, isize
    // u8, u16, u32, u64, u128, usize
    let integer: i32 = 10;
    println!("Integer: {}", integer);

    // f32, f64
    let float: f32 = 10.99;
    println!("Float: {}\n", float);
}
