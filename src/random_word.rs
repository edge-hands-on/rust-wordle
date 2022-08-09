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

// #[cfg(test)]
// mod tests {
//     use crate::matcher::matches;
//     use rstest::rstest;
//
//     #[rstest]
//     #[case("JAZZY", "JAZZY", "11111")]
//     #[case("JAZZY", "CRAZY", "00211")]
//     #[case("CRAZY", "CRAZC", "11110")]
//     #[case("CRAZY", "IIIII", "00000")]
//     fn it_works(#[case] word: String, #[case] guess: String, #[case] output: String) {
//         assert_eq!(matches(word, guess), output)
//     }
// }
