use std::{io, cmp::Ordering};
use rand::{thread_rng as rng, Rng};

fn main() {
    println!("Guess the number!");

    // Create secret number

    let secret_number: u8 = rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // Get user guess (TODO: Refactor to its own function)

    println!("Enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // TODO: Refactor with proper error handling
    
    let guess: u8 = guess.trim().parse().expect("Please type a number!"); // TODO: Refactor to with proper error handling

    // Compare guess with secret number

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Number is smaller!"),
        Ordering::Greater => println!("Number is bigger!"),
        Ordering::Equal => println!("You guessed the number!")
    }
}
