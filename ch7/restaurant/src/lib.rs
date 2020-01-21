
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

use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

// Using nested paths to clean up large use lists
// use std::io;
// use std::cmp::Ordering;
use std::{io, cmp::Ordering};

// Bring all public items into scope with the glob operator
use std::collections::*;

fn serve_order() {}
