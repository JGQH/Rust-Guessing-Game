use std::{io, cmp::Ordering};
use rand::{thread_rng as rng, Rng};

fn main() {
    println!("Guess the number!");

    // Create secret number
    let secret_number: u8 = rng().gen_range(1..=100);

    loop {
        // Get user guess (TODO: Refactor to its own function)
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // TODO: Refactor with proper error handling
        
        let guess: u8 = match guess.trim().parse() {  // TODO: Refactor once inside its own function
            Ok(num) => num,
            Err(_) => {
                println!("Guess is not a valid answer, try again!");
                continue
            }
        };

        // Compare guess with secret number
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is smaller than the secret!"),
            Ordering::Greater => println!("Guess is bigger than the secret!"),
            Ordering::Equal => {
                println!("You guessed the secret!");
                break
            }
        }
    }
}
