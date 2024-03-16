use std::{io, io::Write};

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.trim().to_owned()
}

fn main() {
    let first_word = get_input("Enter the first word: ");
    let second_word = get_input("Enter the second word: ");

    let combined_words = format!("{} {}", first_word, second_word);

    println!("{} {}", first_word, second_word);
    println!("{}", first_word + " " + &second_word);
    println!("{}", combined_words);

    for letter in second_word.chars() {
        print!("{} ", letter);
    }
    println!();

    let letter_replace_from = get_input("Enter the letter to replace: ");
    let letter_replace_to = get_input("Enter the letter to replace with: ");

    let replace_letter = combined_words.replace(&letter_replace_from, &letter_replace_to);

    println!("{}", replace_letter);
}
