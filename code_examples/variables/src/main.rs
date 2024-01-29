// Example with const without main scope
const NUMBER1: i32 = 12;
const NUMBER2: i32 = 34;

fn sum() {
    let result = NUMBER1 + NUMBER2;
    println!("\n{} + {} = {}", NUMBER1, NUMBER2, result);
}

fn main() {
    sum();

    println!("\nMain function \\/\n");

    // Constants
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;

    // Variables
    let mut missiles: i32 = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles);
    println!("{} missiles + ready", missiles + ready * 2);

    // Multiple variables in the same line
    let (var1, mut var2) = (4, 2);

    println!("var1 = {}, var2 = {}", var1, var2);
    var2 = var1 + var2;
    println!("var1 = {}, var2 = {}", var1, var2);

    // Multiple types variables in the same line
    let (var3, mut var4, var5, var6): (i16, f64, char, String) =
        (5, 12.3, 'z', String::from("Zeus"));

    println!(
        "var3 = {}, var4 = {}, var5 = {}, var6 = {}",
        var3, var4, var5, var6
    );

    // NOTE: Since these are variables of different types, it is necessary to perform type conversion.
    var4 = f64::from(var3) + var4;

    println!("var4 = {}", var4);

    // let var7 = 142; // Example unused variable
}
