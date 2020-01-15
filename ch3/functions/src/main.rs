fn main() {
    println!("Hello, world!");

    another_function(5, 6);
}

// in function signatures you must declare the type of each parameter
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}