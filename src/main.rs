use std::io;

fn main() {
    println!("Guess the number!");

    println!("Enter your guess:");

    let mut guess = String::new();  // TODO: Refactor to its own function

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // TODO: Refactor with proper error handling

    println!("You guessed: {guess}");
}
