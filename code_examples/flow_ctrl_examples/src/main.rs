mod mods;

fn main() {
    mods::age_verification::age_verification();
    println!("{}", "-".repeat(20));
    mods::largest_number::find_largest_number();
    println!("{}", "-".repeat(20));
    mods::largest_number::find_largest_number_alternative();
    println!("{}", "-".repeat(20));
    mods::check_even_odd::check_even_odd();
    println!("{}", "-".repeat(20));
    mods::sign_check::check_positive_negative();
}
