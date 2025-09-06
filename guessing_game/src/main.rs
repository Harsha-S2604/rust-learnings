use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("WELCOME TO GUESSING GAME");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You guessed it right!");
                break;
            }
        }
    }
}
