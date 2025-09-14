const BOOKS: [&str; 2] = ["clean code", "The pragmatic programmer"];
pub fn list_books() {
    println!("Listing books");

    for (index, book_title) in BOOKS.iter().enumerate() {
        println!("{0}. {1}", index + 1, book_title);
    }

    println!();
}
