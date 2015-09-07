use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
	input.iter().fold(BTreeMap::new(), |mut tree, (scrabble_value, scrabble_letters)| {
		for letter in scrabble_letters { tree.insert(letter.to_lowercase(), *scrabble_value); }
		tree
	})
}
