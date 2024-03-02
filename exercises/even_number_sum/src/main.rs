fn main() {
    // MODE 1
    let mut sum = 0;

    for number in 0..=100 {
        if number % 2 == 0 {
            sum += number;
        }
    }
    println!("The sum of even numbers from 0 to 100 is: {}", sum);

    // MODE 2
    sum = 0; // Reset value
    for number in (0..=100).step_by(2) {
        sum += number;
    }
    println!("The sum of even numbers from 0 to 100 is: {}", sum);
}
