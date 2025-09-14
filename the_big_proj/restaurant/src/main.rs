mod front_of_house;

use crate::front_of_house::hosting;

fn main() {
    println!("====== Welcome to ABC restaurant ======");
    hosting::add_to_waitlist();
}
