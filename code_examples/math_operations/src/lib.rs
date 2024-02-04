use std::f64::consts;
use std::i32;

pub fn sum(x: i32, y: i32) -> i32 {
    let result = x + y;
    println!("Sum: {} + {} = {}", x, y, result);
    result
}

pub fn subtract(x: i32, y: i32) -> i32 {
    let result = x - y;
    println!("Subtract: {} - {} = {}", x, y, result);
    result
}

pub fn multiply(x: i32, y: i32) -> i32 {
    let result = x * y;
    println!("Multiply: {} * {} = {}", x, y, result);
    result
}

pub fn divide(x: i32, y: i32) -> i32 {
    let result = x / y;
    println!("Divide: {} / {} = {}", x, y, result);
    result
}

pub fn module(x: i32, y: i32) -> i32 {
    let result = x % y;
    println!("Module: {} % {} = {}", x, y, result);
    result
}

pub fn power(x: i32, y: i32) -> i32 {
    let result = i32::pow(x, y as u32);
    println!("Power: {} ^ {} = {}", x, y, result);
    result
}

pub fn float(x: f32, y: f32) -> f32 {
    let result = x + y;
    println!("Float: {} + {} = {}", x, y, result);
    result
}

pub fn sqrt(x: f32) {
    let result = x.sqrt();
    println!("Sqrt: {} = {}", x, result);
}

pub fn sin(x: f32) {
    let sin = x.sin();
    println!("sin: {} = {}", x, sin);
}

pub fn cos(x: f32) {
    let cos = x.cos();
    println!("cos: {} = {}", x, cos);
}

pub fn tan(x: f32) {
    let tan = x.tan();
    println!("tan: {} = {}", x, tan);
}

pub fn exponential(x: f32) {
    let exp = x.exp();
    println!("exp: {} = {}", x, exp);
}

pub fn logarithm(x: f32) {
    let log = x.log10();
    println!("log10: {} = {}", x, log);
}

pub fn const_pi() {
    let pi = consts::PI;
    println!("pi = {}", pi);
}

pub fn const_e() {
    let e = consts::E;
    println!("e = {}", e);
}
