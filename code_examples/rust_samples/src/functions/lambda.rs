use crate::utils::terminal::selected_option;

pub fn map_example() {
    selected_option("Map");

    let pow_base_2: Vec<i32> = (1..=10).map(|x| i32::pow(2, x)).collect();
    println!("Powers of 2: {:?}\n", pow_base_2);
}

pub fn filter_example() {
    let is_prime = |n: i32| -> bool {
        if n < 2 {
            return false;
        }

        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }

        true
    };

    let prime_numbers: Vec<i32> = (1..=1000)
        .filter(|&x| is_prime(x))
        .filter(|&x| x.to_string().ends_with('3'))
        .collect();

    println!("Prime numbers ending with 3: {:?}\n", prime_numbers);
}
