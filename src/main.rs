use std::time::{UNIX_EPOCH, SystemTime};

mod matcher;
mod random_word;
mod game;
mod word_list;

fn main() {
    
    let mut seed = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs();
    
    let mut word_list = word_list::WordList::new("words.txt", seed);
    

    let mut game = game::Game {
        guess_count: 2,
        word_list: word_list
    };

    game.run();
    
}
