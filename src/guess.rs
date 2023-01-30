use std::io::stdin;

pub enum GuessResult {
    Ok(u8),
    IOError,
    ParseError
}

pub fn get_user_guess() -> GuessResult {
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