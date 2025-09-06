fn main() {
    // immutable
    let x = 5;
    println!("The value of x is {x}");
    // x = 10; // throws error as the variable by default immutable
    // println!("The value of x is {x}");
    
    //mutable
    let mut x = 10; // we are doing variable shadowing here
    println!("The value of x is {x}");

    x = 12;
    println!("The value of x is {x}");

    const TOTAL_SECONDS: u32 = 60;
    println!("Total seconds:{TOTAL_SECONDS}");
    
    // overshadowing with different data type
    let x = "Hello x";
    println!("The message is {x}");
}

