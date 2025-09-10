fn main() {
    let s = String::from("Rick and morty");
    let s_len = get_str_length(&s);

    println!("The length of the string '{s}' is {s_len}");

    let mut rm = String::from("Rick and ");
    append_morty(&mut rm, "morty");
    
    let bye: &str = "bye";
    println!("After appending {rm}");

    let mut a = String::from("Rust programming");

    let b = &mut a;
    b.push_str(" language");
    let c = &mut a;
    
    // if you try to use b, it throws an error
    // stating that first borrow later used here.
    //
    // we shouldn't have two mutable references at the same time
    // else it will throw an error.
    //
    // multiple writable references shouldn't be there
    // this is to avoid the data races. Data races occurs in three
    // ways
    // 1. Two or more pointers access the same data at the same time
    // 2. At least one of the pointers is being used to write to the data
    // 3. There's no mechanism being used to synchronize access to the data
    //
    // Here we can use b before declaring the variable c
    println!("{c}");
    
    // but multiple readable references are allowed
    // readable and writable references cannot co-exist
    let mut data = String::from("Hi all!");
    let readable_reference1 = &data;
    let readable_reference2 = &data;
    // the readable_reference1 and readable_reference2 drops off once
    // you create a writable reference and cannot access after
    // writable_reference is created but using it before writable_reference
    // is created won't cause any error.
    println!("{readable_reference1} {readable_reference2}");
    let writable_reference = &mut data;

    let no_dangle_value = no_dangle();
    println!("{no_dangle_value}");
}

fn append_morty(m: &mut String, a: &str) {
    m.push_str(a);
}
fn get_str_length(s: &String) -> usize {
    s.len()
}

// this function throws compilation error
//fn dangle() -> &String {
    //let mut s = String::from("Hello all");
    //&s
//}

fn no_dangle() -> String {
    let mut s = String::from("Hello it's no dangle");
    s
}
