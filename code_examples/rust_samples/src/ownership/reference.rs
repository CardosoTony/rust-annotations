use crate::utils::terminal::selected_option;

pub fn immutable_references() {
    selected_option("Immutable References");

    let text = String::from(
        "\n    'Rust is a systems programming language that runs incredibly fast, prevents segfaults, and guarantees thread safety.'\n",
    );

    let words_count = calculate_word_count(&text);
    println!("The text:{}Has {} words!\n", text, words_count);
}

// Receives a reference to the text (does not take ownership); Borrows the text
fn calculate_word_count(text: &String) -> usize {
    text.split_whitespace().count() // text is dropped (drop is not called because it doesn't have ownership)
}

pub fn mutable_references_01() {
    selected_option("Mutable references");

    let mut name = String::from("Hallie");
    last_name(&mut name);
    println!("Full Name: {}\n", name);
}

// Cannot be borrowed as mutable
fn last_name(name: &String) {
    // name.push_str(" Smith"); error message
    println!("{}", name.len());
}

pub fn mutable_references_02() {
    let mut name = String::from("Jason");
    append_last_name(&mut name);
    println!("Full Name: {}", name);

    let mut_reference_1 = &mut name;
    println!("{}", mut_reference_1);

    let mut_reference_2 = &mut name;
    println!("{}\n", mut_reference_2);
}

// Can be borrowed as mutable
fn append_last_name(name: &mut String) {
    name.push_str(" Smith");
}

pub fn mutable_references_03() {
    let mut name = String::from("Sophia");

    // immutable - no problem
    let immutable_reference_01 = &name;
    let immutable_reference_02 = &name;
    println!("{}, {}", immutable_reference_01, immutable_reference_02);

    // mutable - problem if changing the order
    let mut_reference_03 = &mut name;
    println!("{}\n", mut_reference_03);
}

pub fn dangling_reference() {
    // let points_to_nothing = create_dangling_reference();
    // println!("Dangling Reference {}", points_to_nothing);

    let no_dangle = no_dangle();
    println!("No Dangling - {}\n", no_dangle);
}

/*
fn create_dangling_reference() -> &String {
    let text = String::from("Dangling Reference");
    &text // returns a reference that will be dropped
}
*/

fn no_dangle() -> String {
    let text = String::from("No Dangling Returns Function");
    text // returns a reference that will be moved to the calling function
}
