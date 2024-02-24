/*
Relational Operators
<; >; <=; >=; ==; !=

Logical Operators
&& (AND); || (OR);! (NOT)

Flow Control Structures
    Flow Control Conditionals
        if; else if; else; match
    Flow Control Loops
        while; for; loop
*/

fn main() {
    let x = 5;
    let y = 10;

    println!("x < y: {}", x < y);
    println!("x <= y: {}", x <= y);
    println!("x > y: {}", x > y);
    println!("x >= y: {}", x >= y);
    println!("x == y: {}", x == y);
    println!("x!= y: {}", x != y);
    println!("x == 5: {}", x == 5);
    println!("x!= 5: {}", x != 5);

    println!("{}", "-".repeat(10));

    println!("x && y: {}", x > 0 && y < 15);
    println!("x || y: {}", x > 0 || y < 15);
    println!("x != 10: {}", !(x == 10));

    println!("{}", "-".repeat(10));

    if x == 5 {
        println!("x = {}", x);
    }

    if x == 10 {
        println!("x = {}", x);
    } else {
        println!("x != 10");
    }

    if x == 16 {
        println!("x = {}", x);
    } else if x > y {
        println!("x > y");
    } else {
        println!("x <= y");
    }

    println!("{}", "-".repeat(10));

    let car_model = "GT500";

    match car_model {
        "GT500" => println!("The car model is {}", car_model),
        "Cobra" => println!("The car model is {}", car_model),
        _ => println!("The car model was not found."),
    }

    println!("{}", "-".repeat(10));

    let mut counter_loop = 0;

    loop {
        println!("{}", counter_loop);
        counter_loop += 1;

        if counter_loop >= 5 {
            break;
        }
    }

    println!("{}", "-".repeat(10));

    let mut counter_while = 0;

    while counter_while < 5 {
        println!("{}", counter_while);
        counter_while += 1;
    }

    println!("{}", "-".repeat(10));

    let numbers_for = [2, 3, 4, 5, 6];

    for number in numbers_for.iter() {
        println!("{}", number);
    }

    println!("{}", "-".repeat(10));

    for number_range in 1..=5 {
        println!("{}", number_range);
    }
}
