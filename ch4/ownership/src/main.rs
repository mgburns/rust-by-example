fn main() { // Variable scope starts here.

    // Stack vs. Heap.
    // - Stack is for fixed values known at compile time, and is managed `last in, first out`.
    // - Heap is for values with unknown size, and involves book keeping to manage (pointers, allocation, freeing)
    // - Memory management techniques:
    //   - Garbage Collection. Program automatically looks for unused memory and frees it. (Inefficient!).
    //   - Manual. Programmers explicitly allocate and free memory in their code. (Error prone!)
    //   - Rust = Ownership. A set of ownership rules managed at compile time.

    // String literals are immutable. Memory is allocated at compile time and embedded in the binary.
    let s = "hello";
    println!("{}", s);

    // String types are different. Data is allocated on the heap so that it can be modified at runtime.
    // Memory is automatically allocated on initialization, and dropped when the variable goes out of scope.
    // In C++ this is called Resource Acquisition is Initialization (RAII)
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // Ways that variables and data interact: Move

    // Simple data types are managed on the stack, so this is really just a value copy.
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // Complex data types are different!
    // Rust only allows *one* owner at a time.

    let s1 = String::from("hello"); // s1 owns this value
    let s2 = s1; // s2 takes over ownership
    println!("s2: {}", s2);

    // not allowed! only one owner per heap reference at any given time.
    // println!("s1: {}", s1);
    //                    ^^ value borrowed here after move

    // Shallow vs. Deep copying
    // - Shallow. Just copy pointer.
    // - Deep. Copy values.
    // Rust = No deep copies, courtesy of ownership.

    // Ways that variables and data interact: clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // now this is legal!
    println!("s1: {}", s1);
    println!("s2: {}", s2);
} // Variable scope ends here. All variables defined within are now invalid.