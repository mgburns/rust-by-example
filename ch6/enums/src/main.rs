enum IpAddr {
    // enum variants
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));
}
