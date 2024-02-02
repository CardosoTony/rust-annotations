use std::collections::HashMap; // Necessary for using 'HashMap' in 'Collection types'
use std::collections::HashSet; // Necessary for using 'HashSet' in 'Collection types'
use std::ffi::CStr; // Necessary for using 'CStr' in 'String types'

// Use to ignore the warning message
#[allow(unused_variables)]

fn main() {
    // Numeric types

    // Signed 'integer' types (accepts positive and negative)
    let i8: i8 = -18; // 8 bits
    let i16: i16 = -78; // 16 bits
    let i32: i32 = -21; // 32 bits
    let i64: i64 = -92; // 64 bits
    let i128: i128 = -32; // 128 bits
    let isize: isize = -12; // 64 bits or 32 bits, depending on the system architecture

    // Unsigned 'integer' types (only positive)
    let u8: u8 = 25; // 8 bits
    let u16: u16 = 65; // 16 bits
    let u32: u32 = 42; // 32 bits
    let u64: u64 = 14; // 64 bits
    let u128: u128 = 7; // 128 bits
    let usize: usize = 12; // 64 bits or 32 bits, depending on the system architecture

    // Floating point types (accepts positive and negative)
    let f32: f32 = 3.14; // 32 bits
    let f64: f64 = 2.71; // 64 bits
    let zero = 0.0;

    // Boolean types
    let bool: bool = true; // 1 bit (true or false)

    // Character types
    let char: char = 'a'; // 8 bits (represents a single Unicode character)

    // Compound types (tuples, arrays, slices)

    // Tuple: Is a data structure capable of containing a fixed number of elements, with each element being able to have a different type
    let tup: (i32, u8, f32, bool, char, String, &str) =
        (32, 4, 13.2, true, 'z', String::from("Rise Code"), "Zeus"); // Tuple accepts any types
    println!("Tuple value: {:?}", tup);
    println!("String element: {}", tup.5); // a way to print a specific element

    let tup_index = tup.3; // a way to print a specific element (creating a reference variable)
    println!("Boolean element: {}", tup_index);

    let (a, b, c, d, e, f, g) = tup; // destructuring mode
    println!(
        "Destructuring tuple--> a: {}, b: {}, c: {}, d: {}, e: {}, f: {}, g: {}",
        a, b, c, d, e, f, g
    );
    println!("Specific element--> c: {}", c);

    // Array: Are fixed-size sequences of elements of the same type
    let arr: [i32; 6] = [10, 20, 30, 40, 50, 60]; // 3 elements --> arr: [type; length]
    println!("Array value: {:?}", arr);
    println!("First element: {}", arr[0]);
    println!("Second element: {}", arr[1]);

    // Slices: Slices are references to a contiguous sequence of elements from an array or a similar data structure.
    // They allow access to a specific portion of the array without copying the elements
    let slice = &arr[1..4]; // Returns '20, 30, 40' where index '4' represents the cut-off limit
    println!("Slice value: {:?}", slice);

    // String types
    let s: String = "Hello, world!".to_string(); // 8 bits
    let str: &str = "Hello, world!"; // 8 bits
    let c_str: &CStr = CStr::from_bytes_with_nul(b"Hello, world!\0").unwrap(); // 8 bits
    let mut message_string: String = s;
    message_string.push_str("\nWelcome to Rust!"); // concat string
    println!("String value: {}", message_string);

    // Collection types (Vec, HashMap, HashSet)
    // Vec<T> is a dynamic array that stores elements of the same type 'T'.
    // It allows for efficient addition, removal, and access of elements, and can dynamically grow or shrink in size.
    let mut vector: Vec<i32> = Vec::new(); // Creating a new empty vector
    vector.push(10); // Adding value
    vector.push(20);
    vector.push(30);
    println!("Vector value: {:?}", vector);
    println!("Second element: {}", vector[1]);

    let first_element = vector[0];
    println!("First element: {}", first_element);

    // HashMap<K, V> is a collection of key-value pairs.
    // It allows for efficient insertion, removal, and access of key-value pairs, and can dynamically grow or shrink in size.
    let mut hashmap: HashMap<&str, i32> = HashMap::new(); // Creating a new empty hashmap
    hashmap.insert("key_one", 13); // Adding key-value pair
    hashmap.insert("key_two", 20);
    hashmap.insert("key_three", 43);
    println!("Hashmap value: {:?}", hashmap);

    // Valid if you are sure that the keys exist, however, this can cause panic if the key is not present in the HashMap
    println!("Third key: {}", hashmap["key_three"]);

    // Direct access with expect (to avoid panics)
    println!(
        "Second key: {}",
        hashmap.get("key_two").expect("Key not found")
    );

    let first_element = hashmap.get("key_one").expect("Key not found");
    println!("First key: {}", first_element);

    if let Some(key) = hashmap.get("key_three") {
        println!("Third key: {}", key);
    } else {
        println!("Third key not found");
    }

    // HashSet<T> is a collection of elements of the same type 'T'.
    // It allows for efficient insertion, removal, and access of elements, and can dynamically grow or shrink in size.
    let mut number_set: HashSet<i32> = HashSet::new(); // Creating a new empty set
    number_set.insert(14); // Adding value
    number_set.insert(28);
    number_set.insert(32);
    println!("Set value: {:?}", number_set);

    // Verify that the value is actually present in the set
    let search_number = 2;
    if number_set.contains(&search_number) {
        println!("The number {} is in the set", search_number);
    } else {
        println!("The number {} is not in the set", search_number);
    }

    let mut str_set: HashSet<&str> = HashSet::new();
    str_set.insert("Rust");
    str_set.insert("C++");
    str_set.insert("Ruby");
    str_set.insert("Go");
    println!("Set value: {:?}", str_set);

    let search_string = "Rust";
    if str_set.contains(&search_string) {
        println!("The string \"{}\" is in the set", search_string);
    } else {
        println!("The string \"{}\" is not in the set", search_string);
    }

    // Option<T> is a generic enumeration that represents an optional value.
    // It's used when a value may be present or absent, meaning there is a possibility of a result being null ot empty.
    let result1 = option_example(15.0, 0.0);
    match result1 {
        Some(value) => println!("Result 1: {}", value),
        None => println!("Result 1: Cannot divide by zero."),
    }

    let result2 = option_example(25.0, 5.0);
    if let Some(value) = result2 {
        println!("Result 2: {}", value);
    } else {
        println!("Result 2: Cannot divide by zero.");
    }
}

fn option_example(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}
