fn main() {
    let number = 6;

    // conditions *must* be bool
    if number % 4 == 0 {
        // we've entered a branch or 'arm' of code
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // add as many `else if`'s as you want
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else { // else is optional
        println!("number is not divisible by 2, 3, or 4");
    }
}
