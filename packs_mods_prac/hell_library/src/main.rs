mod library;

pub use crate::library::books;
pub use crate::library::members;

fn main() {
    println!("===== Welcome to HELL library =====\n");
    
    books::list_books();
    members::add_member("John");
}
