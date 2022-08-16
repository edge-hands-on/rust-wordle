use crate::random_word::get_random_word;
use crate::matcher::matches;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{stdin};

mod matcher;
mod random_word;

fn main() {
    let mut guess_count = 6;

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let word = get_random_word("words.txt", seed);
    println!("Chosen word: {}\n", word);

    while guess_count > 0 {
        let mut guess = String::new();
        println!("Please enter your guess: ");
        stdin().read_line(&mut guess).expect("Did not enter a correct string");
        guess = guess.trim().to_string();

        if guess.eq(&word) {
            println!("You win! The word was {}.", word);
            return;
        }
        
        let result = matches(&word, guess);
        println!("Your current result: {}", result);

       guess_count -= 1;
    }
}
