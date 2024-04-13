// // pub fn add(left: usize, right: usize) -> usize {
// //     left + right
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn it_works() {
// //         let result = add(2, 2);
// //         assert_eq!(result, 4);
// //     }
// // }
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         // fn seat_at_table() {}
//     }

//     // mod serving {
//     //     fn take_over() {}
//     //     fn server_order() {}
//     //     fn take_payment() {}
//     // }
// }

// // use crate::front_of_house::hosting;
// use front_of_house::hosting::add_to_waitlist;
// pub use front_of_house::hosting;
// // pub fn eat_at_restaurant() {
// //     crate::front_of_house::hosting::add_to_waitlist();
// //     front_of_house::hosting::add_to_waitlist();
// // }

// use std::{cmp::Ordering, io};
// use std::{self, write};
// fn server_order() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     add_to_waitlist();
//     add_to_waitlist();
//     add_to_waitlist();
//     add_to_waitlist();
// }

// mod back_of_house {
//     // fn fix_incorrect_order() {
//     //     cook_order();
//     //     super::server_order();
//     // }
//     // fn cook_order() {}
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     let order1 = back_of_house::Appetizer::Salad;
//     let order2 = back_of_house::Appetizer::Soup;
// }

// use std::fmt;
// // use std::io;
// use std::collections::*;
// // fn function1() -> fmt::Result {

// // }

// // fn function2() -> io::Result<()> {

// // }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// // fn function1() -> Result {

// // }

// // fn function2() -> IoResult<()> {

// // }

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}