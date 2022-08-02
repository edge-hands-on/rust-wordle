pub fn matches(word: String, guess: String) -> String {
    let mut result = String::from("");
    for (i, word_char) in word.chars().enumerate() {
        let guess_char = guess.chars().nth(i).unwrap();
        if word_char == guess_char {
            result.push('1');
        } else {
            result.push('0')
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::matcher::matches;
    use rstest::rstest;

    #[rstest]
    #[case("JAZZY", "JAZZY", "11111")]
    #[case("JAZZY", "CRAZY", "00211")]
    fn it_works(#[case] word: String, #[case] guess: String, #[case] output: String) {
        assert_eq!(matches(word, guess), output)
    }
}