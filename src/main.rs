use std::{
    io::stdin,
    cmp::Ordering
};
use rand::{thread_rng as rng, Rng};

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

// TODO: Move to its own file
enum GuessResult {
    Ok(u8),
    IOError,
    ParseError
}

fn get_user_guess() -> GuessResult {
    println!("Enter your guess:");
    
    let mut guess = String::new();

    // Return IO error
    if let Err(_) = stdin().read_line(&mut guess) {
        return GuessResult::IOError
    }

    // No IO error, so attempt parsing
    match guess.trim().parse::<u8>() {
        Ok(num) => GuessResult::Ok(num),
        Err(_) => GuessResult::ParseError
    }
}
