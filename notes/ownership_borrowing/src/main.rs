fn main() {
    let name: String = String::from("Rust");
    println!("Hello, {}!", name);

    // Moves the value of String 'name' to String 'new_name'.
    let new_name: String = name;

    // The following code generates an error because 'name' has been moved to 'new_name'.
    //println!("Hello, {}!", name); // Generates an error, borrow of moved value.
    println!("Hello, {}!", new_name);

    let name = String::from("Rust Language");

    // Creates a copy of the contents of String 'name' and stores it in 'new_name'.
    let new_name = name.clone();

    println!("Hello, {}!", name);
    println!("Hello, {}!", new_name);

    // Copy Semantics
    /*
    In Rust, implementing 'Copy' means that a data type can be copied simply, bit by bit, without causing ownership or reference problems. When a type implements 'Copy', every time you assign that type to another variable, you get a complete copy of the data, not a transfer of ownership.

    For example, primitive data types like integers (i32, u64, etc.), characters (char), booleans (bool), and some other simple structures, implement 'Copy'. This means that when you assign these types to another variable, a direct copy of the data occurs.

    However, not all types implement 'Copy'. Types that own resources or depend on other types that do not implement 'Copy', such as String, Vec<T>, and types you create, typically do not implement 'Copy'. When you assign a type that does not implement 'Copy' to another variable, it transfers ownership of the data, which means the original value is no longer valid.
    */

    let list_a = [1, 2, 3, 4, 5];
    let list_b = list_a;

    println!("List a: {:?}", list_a);
    println!("List b: {:?}", list_b);

    let number = 5;
    let another_number = number;

    println!("The value of number is: {}", number);
    println!("The value of another_number is: {}", another_number);
}
