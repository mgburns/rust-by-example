// Storing UTF-8 Encoded Text with Strings
// - Rust has only one core string type -- the sling slice `str`
// - Standard library provides `String`, a growable, miutable, owned, UTF-8 encoded string type
// - When Rustaceans refer to "strings" it usually means both

fn main() {
    // Creating strings
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string(); // Provided by the `Display` trait
    let s = "initial contents".to_string();
    let s = String::from("initial contents"); // equivalent to "".to_string()

    // Updating strings
    let mut s = String::from("foo");
    s.push_str("bar"); // makes "foobar"
    let mut s = String::from("lo");
    s.push('l'); // push a single character -- makes "lol"

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // Invokes the `add` method, signature: fn add(self, s: &str) -> Str
    // Note: s1 has been moved to s3, but s2 is still usable
}
