// All module items (functions, structs, enums, etc.) are private by default
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    // For structs, you have to declare the struct and fields as pub
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Designating an enum as public makes all its variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        // This is required to create a Breakfast instance due to the
        // privacy of the `seasonal_fruit` field.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();

        // `super` is equivalent to `..` here
        super::serve_order();
    }

    fn cook_order() {}
}

// Re-exports for external code as `hosting` with `pub use`
pub use front_of_house::hosting;

use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

// Using nested paths to clean up large use lists
// use std::io;
// use std::cmp::Ordering;
use std::{io, cmp::Ordering};

// Bring all public items into scope with the glob operator
use std::collections::*;

// since this is a sibling of the private front_of_house module, it can use it
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // won't compile! "field `seasonal_fruit` of struct `back_of_house::Breakfast` is private"
    // meal.seasonal_fruit = String::from("blueberries");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Courtesy of `use` above
    // Note: It's idiomatic when bring a function into a scope to
    // use the parent module rather than the function itself to
    // avoid confusion about where the function is declared.
    hosting::add_to_waitlist();

    // For structs, enums, and other items it's idiomatic to
    // specify the full path as demonstrated here.
    let mut map = HashMap::new();
    map.insert(1, 2);

    fn function1() -> Result { Result::Ok(()) }
    fn function2() -> IoResult<()> { IoResult::Ok(()) }
}

fn serve_order() {}
