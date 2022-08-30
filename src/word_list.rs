use std::fs;

use rand::{SeedableRng, rngs::StdRng, Rng};

pub struct WordList<'a> {
    filename: &'a str,
    seed: u64
}

impl WordList<'_> {

    pub fn new (filename: &str, seed: u64) -> WordList {
        WordList {
            filename,
            seed
        }
    }

    pub fn get_random_word(&self) -> String {
        let file_content = fs::read_to_string(self.filename)
            .expect("Something went wrong reading the file");
    
        let word_vector = file_content.split("\n")
            .collect::<Vec<&str>>();
    
        let mut rng: StdRng = SeedableRng::seed_from_u64(self.seed);
    
        return word_vector[rng.gen_range(0..word_vector.len())].trim().to_string();
    }
}
