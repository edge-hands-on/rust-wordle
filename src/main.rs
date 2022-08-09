use crate::random_word::get_random_word;
use std::time::{SystemTime, UNIX_EPOCH};

mod matcher;
mod random_word;

fn main() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    println!("Chosen word: {}\n", get_random_word("words.txt", seed));
}
