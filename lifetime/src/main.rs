fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // this one throws error because rust doesn't know if the returned reference comes from a or b
    if a.len() > b.len() { a } else { b }
}
fn main() {
//    {
//        let x = 5;
//        r = &x; // throws error because x's life time is expired once it's out of the block
//    }

    let s1 = String::from("Hello");
    let s2 = String::from("Worl");

    let result = longest(&s1, &s2);

    println!("{result}");

    /*
     * why lifetime is required for the above one
     * it is to tackle dangling pointers
     *
     * say:
     *  let mut result;
     *  let s1 = String::from("Hello");
     *  {
     *      let s2 = String::from("worl");
     *      result = longest(&s1, &s2);
     *  } 
     *  the result can be either s2 or s1, now since the s2 drops off, we won't have the
     *  reference to s2 and it throws error stating that lifetime of s2 is 0 when it comes
     *  out of the block and result will be dangling pointer
     *
     * */
}
