/*
    This program generates a random number and asks the user to guess the
    value generated.

    The code is based on this tutorial from the Rust Book:
    https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
*/

use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    //Generates number with the rand library crate
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    // Game loop, will continue as long as the user's guess is incorrect
    loop {
        println!("Please input your guess.");

        // Mutable variable to store user input
        let mut guess = String::new();
        
        // Stores user input in variable guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Attempts to convert the user input to a u32 integer. If the
        // input is invalid, the user is asked to input again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Prints guess to console
        println!("You guessed: {guess}");

        // Compares user input to generated number
        match guess.cmp(&secret_number){
            // Prints if guess is smaller than number
            Ordering::Less => println!("Too small!"),
            // Prints if guess is larger than number
            Ordering::Greater => println!("Too big!"),
            // If guess matches number, prints "You win!" and ends
            // the program
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
