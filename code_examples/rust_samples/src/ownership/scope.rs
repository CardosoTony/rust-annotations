use crate::utils::terminal::selected_option;

pub fn basic_example() {
    selected_option("Basic Example");
    {
        let string = String::from("Hello!");
        println!("{}\n", string);
    }
    // println!("{}", string); // error message
}

pub fn lifetime_example() {
    selected_option("Lifetime Example");

    let x;
    {
        let inner_scope = String::from("Inner Scope");
        x = &inner_scope;

        println!("{}, {}", x, inner_scope);
        println!("{}\n", x);
    }
    // println!("{}", x); // error message in 'x = &inner_scope;'
}

pub fn move_example() {
    selected_option("Move Example");

    // Value is allocated on the stack
    let number1 = 18;
    let number2 = number1;
    println!("number1 = {}, number2 = {}", number1, number2);

    // Value is allocated on the heap
    let string1 = String::from("Hello!");

    // 'string1' has been moved to 'string2'
    let string2 = string1;

    // println!("{}", string1); // error message
    println!("{}\n", string2);
}

pub fn clone_example() {
    selected_option("Clone Example");

    let string1 = String::from("Hello!");

    // Clone needs to be explicitly called
    let string2 = string1.clone();

    println!("{} {}\n", string1, string2);
}

pub fn copy_example() {
    selected_option("Copy Example");

    // Fixed values are stored on the stack and are copied. Needs to implement the 'Copy trait'. (i32, f64, bool, char, tuples with Copy types)
    let list_a = [1, 2, 3, 4, 5];
    let list_b = list_a;

    println!("list_a = {:?}", list_a);
    println!("list_b = {:?}\n", list_b);
}
