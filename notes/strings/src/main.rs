fn main() {
    println!("Starting...");

    // Creates a new empty String
    let mut empty_string_1 = String::new();
    let mut empty_string_2 = String::new();

    println!("{}", empty_string_1); // prints an empty line
    println!("{}", empty_string_2); // prints an empty line

    // Defines string slices
    let string_slice_1 = "Hello";
    let string_slice_2 = "Hello";

    println!("{}", string_slice_1); // prints "Hello, "
    println!("{}", string_slice_2); // prints "Hello, "

    // Converts string slices into Strings
    empty_string_1 = string_slice_1.to_string();
    empty_string_2 = String::from(string_slice_2);

    println!("{}", empty_string_1); // prints "Hello, "
    println!("{}", empty_string_2); // prints "Hello, "

    let name = "World!";

    // Appends a string slice to the end of the string
    empty_string_1.push_str(", ");
    empty_string_1.push_str(name);

    println!("{}", empty_string_1); // prints "Hello, World!"

    // Concatenates strings using the '+' operator
    empty_string_2 = empty_string_2 + ", " + name;

    println!("{}", empty_string_2); // prints "Hello, World!"

    let name = "Christine";
    let age = 34;

    // Uses 'format!' macro to create a new string with formatted values
    let format_string = format!("Hello, {}! You are {} years old.", name, age);

    println!("{}", format_string); // prints "Hello, Christine! You are 34 years old."

    // Gets the length if the 'format_string'
    let string_length = format_string.len();

    println!("Length of the string: {} characters", string_length); // prints '... 39 ...'

    // Slices the 'format_string' to extract parts of it
    let slice_1 = &format_string[7..16];
    let slice_2 = &format_string[26..34];

    println!("{}", slice_1); // prints "Christine"
    println!("{}", slice_2); // prints "34 years"
}
