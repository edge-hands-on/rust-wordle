pub fn matches(word: String, guess: String) -> String {
    let mut result = String::from("");
    for (i, word_char) in word.chars().enumerate() {
        let guess_char = guess.chars().nth(i).unwrap();
        if word_char == guess_char {
            result.push('[');
            result.push(word_char);
            result.push(']');
        } else {
            result.push(guess_char)
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::matcher::matches;

    #[test]
    fn it_works() {
        assert_eq!(
            matches(String::from("JAZZY"), String::from("CRAZY")),
            String::from("CR(A)[Z][Y]")
        )
    }
}