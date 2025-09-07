fn main() {
    // branches
    let num = 3;
    
    // condition must be bool always or else we will get an error
    // for example:
    //  if num {
    //      println!("The value is {num}");
    //  } else {
    //      println!("Invalid number");
    //  }
    //
    //  the above code will throw an error because it's not evaluating
    //  to the bool
    //
    //  It throws "expected a bool but got an integer
    if num == 3 {
        println!("The value is 3");
    } else {
        println!("The value is not 3");
    }

    // using if in a let statement
    let condition = false; 
    let number = if condition { 5 } else { 10 };
    println!("the value of number is {number}");

    // repeating code with loop
    
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 100 {
            break counter;
        }
    };

    // label break
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 20;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up; 
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("the counter is {result}");
    
    // for loop
    println!();
    println!("==== FOR LOOP ===="); 
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for element in arr {
        println!("The element is {element}");
    }

    // for range
    println!();
    println!("==== FOR RANGE LOOP ====");
    for element in 1..4 {
        println!("The element is {element}");
    }

    // reverse range
    println!();
    println!("==== FOR REVERSE RANGE LOOP ====");

    for element in (1..6).rev() {
        println!("The element is {element}");
    }
}
