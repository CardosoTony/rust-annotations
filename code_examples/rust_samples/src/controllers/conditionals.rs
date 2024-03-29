use crate::utils::terminal::selected_option;

pub fn example() {
    selected_option("Conditional");
    let x = 12;
    let y = 23;

    if x > y {
        println!("x > y");
    } else if x < y {
        println!("x < y");
    } else {
        println!("x = y");
    }
    println!();
}
