fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world!");
   
    // hello_world is borrowing part of data from s
    let hello_word = &s[0..5];
    println!("{hello_word}");
    let first_word = get_first_word(&s);
    println!("The first word is {first_word}"); 
    s.clear();
}
