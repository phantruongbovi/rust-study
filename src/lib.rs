// mod front_house {
//     pub mod hosting {
//         pub fn add_to_waiting() {}
//         pub fn seat_at_table() {}
//     }

//     pub mod serving {
//         pub fn take_order() {}
//         pub fn take_payment() {}
//     }
// }

// fn call_order() {}

// mod back_house {
//     use crate::call_order;

//     fn cook_order() {}
//     fn fix_order() {
//         call_order();
//         cook_order();
//     }
// }

// fn eat_to_res() {
//     // crate === self
//     // if in one file -> do not use crate (self)
//     crate::front_house::hosting::add_to_waiting();
// }
use rand::{Rng, RngCore};

mod back_house {
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }

    pub enum Salad {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("banana"),
            }
        }
    }
}

mod front_house;

fn eat_res() {
    front_house::hosting::add_to_waitlist();
}

fn eat_at_res() {
    let mut order = back_house::Breakfast::monday("Fish");
    order.toast = String::from("Chicken");

    let salad = back_house::Salad::Salad;
}
