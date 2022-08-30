use std::{time::{SystemTime, UNIX_EPOCH}, io::stdin};
use crate::matcher::matches;
use crate::word_list::WordList;


use crate::random_word::get_random_word;



pub struct Game<'a> {
    pub guess_count: u32,
    pub word_list: WordList<'a>,
}

impl Game<'_> {
   pub fn run(&mut self) {

        let word = self.word_list.get_random_word();
        println!("Chosen word: {}\n", word);

        while self.guess_count > 0 {
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
    
           self.guess_count -= 1;
        }
    }
}