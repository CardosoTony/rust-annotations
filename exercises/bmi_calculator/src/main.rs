use std::io;

fn main() {
    println!("\nEnter your weight [kg]: ");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Error reading");
    let weight: f32 = weight.trim().parse().expect("Error parsing");

    println!("\nEnter your height [m]: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Error reading");
    let height: f32 = height.trim().parse().expect("Error parsing");

    let bmi = bmi(weight, height);
    let bmi_value = (bmi * 10.0) as i32;

    let result = match bmi_value {
        ..=185 => "Underweight",
        ..=249 => "Normal",
        ..=299 => "Overweight",
        ..=349 => "Obesity I",
        ..=399 => "Obesity II",
        _ => "Obesity III",
    };

    println!("\nBMI: {:.1} {}\n", bmi, result);
}

fn bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}

/*
BMI Classification
0 - 18.5 -> Underweight
18.5 - 24.9 -> Normal weight
25 - 29.9 -> Overweight
30 - 34.9 -> Obesity Grade I
35 - 39.9 -> Obesity Grade II
40 -> Obesity Grade III

result[kg/m²]
*/
