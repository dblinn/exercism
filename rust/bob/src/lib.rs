pub fn reply<'a>(prompt: &'a str) -> &'a str {
    if prompt.is_empty() { "Fine. Be that way!" }
    else if prompt.chars().last().unwrap() == '?' { "Sure." }
    else if prompt.chars().all(|c| !c.is_lowercase()) { "Whoa, chill out!" }
    else { "Whatever." }
}
