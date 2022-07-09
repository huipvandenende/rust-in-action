fn main() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);

    // Print contents of vector.
    println!("{:?}", vector);

    assert_eq!(vector.len(), 5);
    assert_eq!(vector[0], 1);
}
