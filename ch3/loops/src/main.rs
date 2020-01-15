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
}
