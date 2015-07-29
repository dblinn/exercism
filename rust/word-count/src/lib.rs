use std::collections::HashMap;

fn to_lowercase(word: &str) -> String {
	let mut lower = String::new();
	for c in word.chars().flat_map(char::to_lowercase) { lower.push(c); }

	lower
}

pub fn word_count(s: &str) -> HashMap<String, u32> {
	let mut results = HashMap::<String, u32>::new();
	for field in s
		.split(|c: char| !c.is_alphanumeric())
		.filter(|s| !s.is_empty())
	{
		let key = to_lowercase(field);
		let value = results.entry(key).or_insert(0);
		*value += 1;
	}

	results
}