fn main() {
    /*
        Arrays are denoted by [T; N], where:
            T -> represents the type of elements,
            N -> represents the number of elements.
    */

    // Create an array of size 1x5
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Create a 2-dimensional matrix with dimensions 2x3
    let b: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("Accessing the elements of the array");
    println!("a[0] = {}", a[0]);
    println!("a[3] = {}", a[3]);

    println!("Accessing the elements of the 2-dimensional matrix");
    println!("b[0][1] = {}", b[0][1]);
    println!("b[1][2] = {}", b[1][2]);

    println!("Iterating over the array");
    for i in &a {
        println!("a[{}] = {}", i, i);
    }
    // Alternatively, in a simple manner
    for i in &a {
        println!("Element: {}", i);
    }

    println!("Iterating over the 2-dimensional matrix");
    for (i, line) in b.iter().enumerate() {
        for (j, &element) in line.iter().enumerate() {
            println!("b[{},{}] = {}", i, j, element);
        }
    }
    // Alternatively, in a simple manner
    for line in &b {
        for element in line {
            println!("{}", element);
        }
    }
}
