// Rust has three kinds of loops -- loop, while, and for.
fn main() {
    // loops indefinitely, ctrl-C to interrupt
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // break will exit the loop, and optional return the value of the expression that follows
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;

    // while is like loop but with a condition
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("liftoff!");

    let a = [10, 20, 30, 40, 50];

    // this could be accomplished with a `while` loop, but `for` is cleaner
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // `for` is more idiomatic than `while` in most situations.
    for number in (1..4).rev() { // (1..4) generates a Range, a type provided by the standard library
        println!("{}!", number);
    }
    println!("liftoff!");
}
