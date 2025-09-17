#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

impl User {
    fn new(name: &str, email: &str) -> Self {
        let name = String::from(name);
        let email = String::from(email);

        Self {
            name,
            email,
        }
    }
}

use std::collections::HashMap;

fn main() {
    let user1 = User::new("john", "john@gmail.com");

    let mut hash_map = HashMap::new();
    hash_map.insert(String::from("1"), &user1);

   for (key, value) in hash_map {
       println!("{key}: {:#?}", value);
   }

    let text = "Hello world wonderful world";
    let mut word_freq = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_freq.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", word_freq);
}
