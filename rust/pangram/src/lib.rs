use std::collections::HashSet;

pub fn is_pangram(str: &str) -> bool {
    let mut alphabet : HashSet<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for c in str.chars().flat_map(char::to_uppercase) {
        alphabet.remove(&c);
    }

    alphabet.is_empty()
}
