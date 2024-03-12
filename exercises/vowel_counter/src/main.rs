use std::{io, io::Write};

fn main() {
    print!("Enter a phrase: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.");

    let vowels = "aeiouAEIOU";
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for c in input.chars() {
        if c.is_alphabetic() {
            if vowels.contains(c) {
                vowel_count += 1;
            } else {
                consonant_count += 1;
            }
        }
    }

    println!("The number of vowels in the phrase is: {}", vowel_count);
    println!(
        "The number of consonants in the phrase is: {}",
        consonant_count
    );
}
