use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
	input.iter().fold(BTreeMap::new(), |tree, (scrabble_value, ref scrabble_letters)| {
		scrabble_letters.iter().fold(tree, |mut inner_tree, letter| {
			*inner_tree.entry(letter.to_lowercase()).or_insert(0) = *scrabble_value;
			inner_tree
		})
	})
}
