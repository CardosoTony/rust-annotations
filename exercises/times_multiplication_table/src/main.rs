fn main() {
    for multiplier in 1..=10 {
        println!("The multiplication table for {} is: ", multiplier);
        for multiplicand in 1..=10 {
            let product = multiplier * multiplicand;
            println!("{} x {} = {}", multiplier, multiplicand, product);
        }
        println!("{}", "-".repeat(10));
    }
}
