enum IpAddr {
    // enum variants
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit, // no data
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String), // String
    ChangeColor(i32, i32, i32), // Three integers
}

impl Message {
    // enums can have methods, just like structs
    fn call(&self) {}
}

fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option Enum
    // Rust has no `null` value. Instead it has the Option<T> enum.
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // How do we use the <T> in Option<T>? `match`!

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            // we can't omit this! it won't compile as matches are exhaustive.
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Some(5) -> {:?}", five);
    println!("plus_one(Some(5)) -> {:?}", six);
    println!("plus_one(None) -> {:?}", none);

    // The _ Placeholder
    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // unit value, nothing will happen
    }

    // Concise Control Flow with if let
    // Syntactic sugar for a match that only cares about 1 value
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
