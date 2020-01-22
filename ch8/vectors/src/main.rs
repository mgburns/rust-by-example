fn main() {
    // Vec<T>
    // Allows you to store more than one value in a single data structure that puts all the values next to each other in memory.

    // Create vectors with the Vec::new() function (requires type annotation) or the vec! macro (infers type from initial values)
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Update mutable vectors with the push method
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    // Dropping a vector drops its elements
    // Vectors are freed when they go out of scope, like any other struct

    // Reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];

    // Will panic if index is out of bounds
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Returns an Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // Vectors follow the typical borrowing rules like all other references
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6);
    // ^^^^^^^^^ mutable borrow occurs here

    println!("The first element is {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * is the dereferencing operator
    }

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}
