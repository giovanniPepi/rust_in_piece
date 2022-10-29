use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please, input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            // Read line returns a Result value
            // if this istance of Result is an Err value, expect will cause the program to crash and display the error message to .expect
            .expect("Failed to read line");

        // converts the guess to 32-bit number;
        let guess: u32 = match guess.trim().parse() {
            // parse returns a Result type and Result is an enum that has the variants Ok and Err
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
        }
    }
}
