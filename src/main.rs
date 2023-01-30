mod guess;

use std::cmp::Ordering;
use rand::{thread_rng as rng, Rng};
use crate::guess::{
    GuessResult,
    get_user_guess
};

fn main() {
    println!("Guess the number!");

    // Create secret number
    let secret_number: u8 = rng().gen_range(1..=100);

    loop {
        match get_user_guess() {
            GuessResult::Ok(guess) => match guess.cmp(&secret_number) {
                Ordering::Less => println!("Guess is smaller than the secret!"),
                Ordering::Greater => println!("Guess is bigger than the secret!"),
                Ordering::Equal => {
                    println!("You guessed the secret!");
                    break
                }
            },
            GuessResult::ParseError => {
                println!("Guess is not a valid answer, try again!");
                continue
            },
            GuessResult::IOError => {
                println!("Failed to read line, try again.");
                continue
            }
        };
    }
}