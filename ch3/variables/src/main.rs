fn main() {
    // Immutability and shadowing
    let x = 5; // immutable by default
    let x = x + 6; // this is legal due to shadowing, whereas `x = x + 6` is not unless defined as `let mut x = 5`

    // Integers
    let i = 1; // i32 by default
    let i: u64 = 2;
    let i: usize = 3; // architecture dependency unsigned

    // Floats (IEEE-754)
    let f = 2.0; // f64 by default, double precision
    let f: f32 = 3.0; // single precision

    // Boolean (stored as a single byte)
    let b = true;
    let b: bool = false;

    // Characters (four byte unicode scalar)
    let c = 'z';
    let c = '😻';
    println!("{}", c);

    // Complex

    // Tuple
    let t = (500, 6.4, 'c'); // Accommodate mixed types
    let (a, b, c) = t; // Destructuring
    println!("({}, {}, {})", a, b, c);
    println!("({}, {}, {})", t.0, t.1, t.2); // Access via numeric index

    // Array
    // - Unlike tuples elements must have the same type
    // - Useful when you want data allocated on stack rather than heap
    // - Fixed number of elements, unlike vector (provided by standard library)
    let a = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    println!("The third month is: {}", months[3 - 1]);
}
