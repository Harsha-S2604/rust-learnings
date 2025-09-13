fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let x = Some(21);
    let none = None;

    let tt = plus_one(x);
    let n = plus_one(none);
    
    match tt {
        None => println!("Please provide some value"),
        Some(n) => {
            println!("The value of n is {}", n);
            let t = n + 1;
            println!("The value after adding is {}", t);
        },
    }

    match n {
        None => {
            println!("Please provide some value");
        },
        _ => {
            println!("N has some value");
        }
    }

}
