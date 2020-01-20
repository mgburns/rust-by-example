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
}
