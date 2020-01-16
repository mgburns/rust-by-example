fn main() {
    // Ownership and Functions.
    let i1 = 5;
    let s1 = String::from("hello");

    // allowed...
    println!("{}", i1);
    println!("{}", s1);

    take_ownership(s1);
    make_copy(i1);

    // still allowed...
    println!("{}", i1);

    // no longer allowed! the function has taken over ownership of the value previously refered to by s
    // and Rust has therefore invalidated s.
    // println!("{}", s1);
    println!("{}", s2);

    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = takes_and_gives_ownership(s1);
    // println!("{}", s1);
    //                ^^ value borroed here after move
    println!("{}", s2);

    // while this is safe, it's also tedious to always pass back a value so you can use it afterwards.
    let s = String::from("hello");
    let (s, len) = calculate_length(s);
    println!("String: {} Length: {}", s, len);

    // we can avoid this by passing values by reference, which bypasses ownership rules.
    // Rust calls this pattern (reference as function parameters) "borrowing"
    let s = String::from("hello");
    let len = calculate_length_by_ref(&s);
    println!("String: {} Length: {}", s, len);

    // what if we want to modify a value passed by reference? use a mutable reference!
    let mut s = String::from("hello"); // initial variable is a mut String
    change(&mut s); // function must accept a &mut String
    println!("{}", s);

    // one big restriction here -- there can only be 1 mutable reference in any given scope.
    let mut s = String::from("hello");
    let m1 = &mut s;
    // let m2 = &mut s;
    //          ^^^^^^ second mutable borrow occurs here
    // println!("{}, {}", m1, m2);
    //                    -- first borrow later used here

    // Major benefit -- Rust can prevent data race conditions at compile time! 😲

    // Note -- you can have as many immutable references as you want, as there's no risk of data being changed.
    let s = String::from("Hello");
    let s1 = &s;
    let s2 = &s;
    println!("perfectly legal! {}, {}", s1, s2);

    // Dangling references
    // Occurs when there's a pointer to a memory location that was freed elsewhere. Not possible in Rust.
    // let s = dangle();
    let s = no_dangle();
    println!("no dangle! {}", s);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
} // s goes out of scope here, but since it does not have owernship of what it refers to nothing happens

fn change(some_string: &mut String) {
    // modifying a borrowed parameter requires a &mut type declaration.
    some_string.push_str(", world");
}

// This function, which would produce a dangling reference, will not compile!
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // just return the String directly!
