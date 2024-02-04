use math_operations::{
    const_e, const_pi, cos, divide, exponential, float, logarithm, module, multiply, power, sin,
    sqrt, subtract, sum, tan,
};

fn main() {
    let x = 15;
    let y = 5;
    let float1: f32 = 2.5;
    let float2: f32 = 4.0;

    sum(x, y);
    subtract(x, y);
    multiply(x, y);
    divide(x, y);
    module(x, y);
    power(x, y);
    float(float1, float2);
    sqrt(float1);
    sin(float1);
    cos(float2);
    tan(float1);
    exponential(float1);
    logarithm(float2);
    const_pi();
    const_e();
}
