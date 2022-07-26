pub fn matches(_word: String, _guess: String) -> String {
    return String::from("WRONG")
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