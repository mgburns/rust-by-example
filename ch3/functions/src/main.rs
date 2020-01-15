fn main() {
    // Rust is more explicit with statements v. expressions than other languages.
    let y = {
        let x = 3; // let x is a statement, 3 is an expression that resolves to the value 3
        x + 1 // expressions don't include ending semicolons, so this will be assigned to y
    }; // this {} block is an expression evaluating to 4

    another_function(5, 6);
    println!("Five: {}", five());
    println!("Five plus one: {}", plus_one(5));
}

// in function signatures you must declare the type of each parameter
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// return values must have a declared type, but don't need a name
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}