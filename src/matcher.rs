pub fn matches(word: String, guess: String) -> String {
    let mut word_mask: [char; 5] = ['0'; 5];
    let mut guess_mask: [char; 5] = ['0'; 5];
    for (i, guess_char) in guess.chars().enumerate() {
        let word_char = word.chars().nth(i).unwrap();
        if guess_char == word_char {
            guess_mask[i] = '1';
            word_mask[i] = '1';
        }
    }
    for (i, guess_char) in guess.chars().enumerate() {
        if guess_mask[i] == '0' {
            for (j, word_char) in word.chars().enumerate() {
                if word_mask[j] == '0' && guess_char == word_char {
                    guess_mask[i] = '2';
                    word_mask[j] = '2';
                }
            }
        }
    }
    return guess_mask.iter().collect();
}

#[cfg(test)]
mod tests {
    use crate::matcher::matches;
    use rstest::rstest;

    #[rstest]
    #[case("JAZZY", "JAZZY", "11111")]
    #[case("JAZZY", "CRAZY", "00211")]
    #[case("CRAZY", "CRAZC", "11110")]
    #[case("CRAZY", "IIIII", "00000")]
    fn it_works(#[case] word: String, #[case] guess: String, #[case] output: String) {
        assert_eq!(matches(word, guess), output)
    }
}
