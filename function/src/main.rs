fn main() {
    hello_x(12);
    print_measurable_param(12, 'P');
    let y = get_five();
    println!("The value of y is {y}");

    let z = {
        let x = 12;
        x + 1
    };
    println!("The value of z is {z}");
}

fn print_measurable_param(x: i32, c: char) {
    println!("The measurement is: {x}{c}");
}

fn get_five() -> u8 {
    5
}

fn hello_x(x: i32) {
    println!("The value of x is {x}");
}
