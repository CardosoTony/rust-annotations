use crate::utils::terminal::selected_option;

pub fn example() {
    selected_option("Slice");

    let text = String::from("Rust is proving to be a productive tool for collaborating among large teams of developers with varying levels of systems programming knowledge");
    println!("Full text: {}", text);

    let word = first_word_with('d', &text);
    println!("\nThe word is: '{}'\n", word);
}

fn first_word_with(letter: char, slice: &str) -> &str {
    for word in slice.split_whitespace() {
        if let Some(c) = word.chars().next() {
            if c == letter {
                return word;
            }
        }
    }
    slice
}
