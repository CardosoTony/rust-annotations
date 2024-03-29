use crate::utils::terminal::selected_option;

pub fn example() {
    selected_option("Sequences");

    let tuple = (4, 5, 6);
    println!("Tuple: {:?}", tuple);
    println!("Tuple.2: {}", tuple.2);

    let (a, b, c) = tuple;
    println!("Tuple Elements => a: {}, b: {}, c: {}", a, b, c);

    let mut array: [i32; 10] = [0; 10];
    println!("Array: {:?}", array);

    array[0] = 2;
    array[3] = 7;
    array[9] = 13;
    println!("Array: {:?}", array);

    println!("Array[4]: {}", array[4]);

    // Dynamically Sized Type (DST)
    let mut slice: &[i32] = &array[2..6];
    println!("Slice: {:?}", slice);

    slice = &array[4..8];
    slice.iter().for_each(|x| print!("{} ", x));
    println!();

    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    println!("Vec: {:?}", vector);

    let mut vector = vec![5, 6, 7, 8];
    vector.push(9);
    println!("Vec[1]: {}", vector[1]);
    println!("Vec.len(): {}", vector.len());
    println!("Vec.pop(): {}", vector.pop().unwrap());
    println!("Vec.len(): {}\n", vector.len());
}
