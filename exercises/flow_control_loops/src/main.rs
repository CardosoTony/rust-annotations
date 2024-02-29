fn main() {
    println!("While mode:");
    let mut number = 1;
    while number <= 10 {
        print!("{} ", number);
        number += 1;
    }
    println!();
    println!("{}", "-".repeat(20));

    println!("For mode:");
    for number in 1..=10 {
        print!("{} ", number);
    }
    println!();
    println!("{}", "-".repeat(20));

    println!("While mode: 0 to 100");
    let mut number = 0;
    while number <= 100 {
        print!("{} ", number);
        number += 1;

        if number % 10 == 0 {
            println!();
        }
    }
    println!();
    println!("{}", "-".repeat(20));

    println!("For mode: 1 to 100 and only even numbers");
    for number in 1..=100 {
        if number % 2 == 0 {
            print!("{} ", number);
        }

        if number % 10 == 0 {
            println!();
        }
    }
    println!("{}", "-".repeat(20));

    println!("For mode: 1 to 100 and only odd numbers");
    for number in (1..=100).step_by(2) {
        print!("{} ", number);
        if number % 9 == 0 {
            println!();
        }
    }
    println!("{}", "-".repeat(20));
}
