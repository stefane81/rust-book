mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// static path
// use crate::front_of_house::hosting;

// relative path
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {}
