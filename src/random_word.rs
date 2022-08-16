use std::fs;
use std::ops::Range;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub fn get_random_word(filename: &str, seed: u64) -> String {
    let file_content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let word_vector = file_content.split("\n")
        .collect::<Vec<&str>>();

    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    return word_vector[rng.gen_range(0..word_vector.len())].trim().to_string();
}

#[cfg(test)]
mod tests {
    use crate::random_word::get_random_word;
    use rstest::rstest;

    #[rstest]
    #[case("words.txt", 13, "musks")]
    #[case("words.txt", 112400000, "ocean")]
    fn test_get_random_word(#[case] filename: &str, #[case] seed: u64, #[case] output: String) {
        assert_eq!(get_random_word(filename, seed), output)
    }
}
