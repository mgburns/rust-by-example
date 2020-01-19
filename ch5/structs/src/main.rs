fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);

    // *all* struct fields inherit mutability from instance
    let mut user2 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    user2.email = String::from("anotheremail@example.com");
    println!("{:?}", user2);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // struct update syntax (think es6 object destructuring)
        // note: no change in ownership as it copies values into new instance
        ..user1
    };
    println!("{:?}", user3);

    // Tuple struct instantiation
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand (think es6 prop value shorthand)
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs -- like tuples, but with a name
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
