use crate::utils::terminal::selected_option;

pub fn functions() {
    selected_option("Function");

    basic_function();

    function_with_parameters(15, 32);

    let result = function_with_return();
    println!("Return with return: {}", result);

    let result = function_with_return_and_parameters(15, 32);
    println!("Return with parameters: {}\n", result);
}

fn basic_function() {
    println!("Basic function")
}

fn function_with_parameters(x: i32, y: i32) {
    println!("Function with parameters: x = {}, y = {}", x, y);
}

fn function_with_return() -> i32 {
    17
}

fn function_with_return_and_parameters(x: i32, y: i32) -> i32 {
    x + y
}
