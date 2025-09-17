fn main() {
    let mut foo = String::from("foo");
    let bar = "bar";
    foo.push_str(bar);

    println!("updated string is: {:?}", foo);
    println!("Added string is: {bar}");

    let mut lo = "lo".to_string(); // same as String::from
    let l = 'l';
    lo.push(l);

    println!("The text is {lo}");

    // combining using + operator
    // it uses add method which signature looks like below
    // fn add(Self, str: &s)
    // here we are passing the ownership of Self and borrowing the second
    // string
    let hello = "hello".to_string();
    let world = "world".to_string();
    let hello_world = hello + &world;

    println!("{hello_world}");

    let s1 = "hello";
    let first_char_s1 = &s1[0];
    println!("{first_char_s1}");
}
