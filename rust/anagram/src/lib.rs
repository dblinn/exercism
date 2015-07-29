fn to_lowercase(word: &str) -> Vec<char> {
	word.chars().flat_map(char::to_lowercase).collect::<Vec<char>>()
}

fn is_anagram(lowercase_word: &Vec<char>, sorted_word: &Vec<char>, candidate: &str) -> bool {
	let lowercase_candidate = to_lowercase(candidate);
	if lowercase_word == &lowercase_candidate { return false; }

	let mut sorted_candidate = lowercase_candidate.clone();
	sorted_candidate.sort();
	sorted_word == &sorted_candidate
}

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
	let lowercase_word = to_lowercase(word);
	let mut sorted_word = lowercase_word.clone();
	sorted_word.sort();

	candidates.iter()
		.filter(|c| is_anagram(&lowercase_word, &sorted_word, c))
		.map(|c| *c)
		.collect()
}