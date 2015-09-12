pub fn reply<'a>(prompt: &'a str) -> &'a str {
    if prompt.is_empty() { "Fine. Be that way!" }
    else if is_question(prompt) { "Sure." }
    else if is_shouting(prompt) { "Whoa, chill out!" }
    else { "Whatever." }
}

fn is_question(prompt: &str) -> bool {
    prompt.ends_with('?')
}

fn is_shouting(prompt: &str) -> bool {
    prompt.chars().all(|c| !c.is_lowercase())
}
