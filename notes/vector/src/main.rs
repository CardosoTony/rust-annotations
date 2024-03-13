fn main() {
    // Creating a vector:
    // To create an empty vector.
    let mut empty_vector1: Vec<String> = Vec::new();
    // or
    let mut empty_vector2: Vec<u32> = Vec::new();

    // To create a vector initialized with elements.
    let initialized_vector = vec![2, 4, 6, 8, 10];

    // Adding elements to the vector:
    let mut vector1 = vec![1, 3, 5];
    vector1.push(7); // [1, 3, 5, 7]
    vector1.extend([9, 11, 13].iter()); // [1, 3, 5, 7, 9, 11, 13]

    // Accessing elements of the vector:
    println!("Index 4 | Element {}", vector1[4]);

    // Size and iteration of the vector:
    println!("Size of the vector: {}", vector1.len());

    // Loop used to iterate over the elements of the vector.
    for elem in &vector1 {
        println!("Element: {}", elem);
    }

    // Removing elements from the vector:
    println!("Before removing element: {:?}", vector1);
    // Remove the last element from the vector.
    vector1.pop();
    println!("After removing the last element: {:?}", vector1);
    // Remove the specific element from the vector.
    vector1.retain(|&x| x != 7);
    println!("After removing the specific element: {:?}", vector1);

}
