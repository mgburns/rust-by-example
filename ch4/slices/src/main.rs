fn main() {
    // Goal: write a function that returns the first word of a string, or the whole word if there are no spaces.

    // First (fragile) attempt -- return the index following the last character of the first word.
    let mut s = String::from("hello world");
    let i = first_word_fragile(&s); // will return 5
    println!("first word: {}", &s[..i]);

    s.clear();

    // runtime error!
    // println!("first word: {}", &s[..i]);
    // thread 'main' panicked at 'byte index 5 is out of bounds of ``'

    // the value `5` is decoupled from the String reference, and can therefore become invalid (as it is now)

    // Solution: Slices

    // Basic example
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let mut s = String::from("hello world");
    let word = first_word_robust(&s); // will get 5
    println!("first word: {}", word);

    // won't compile!
    // s.clear();
    // ^^^^^^^^^ mutable borrow occurs here
}

fn first_word_fragile(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn first_word_robust(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
