use std::collections::HashMap;

pub fn nucleotide_counts(seq: &str) -> HashMap<char, usize> {
	let mut results = HashMap::<char, usize>::new();
	for acid in ['A', 'T', 'C', 'G'].iter() {
		results.insert(*acid, count(*acid, seq));
	}

	results
}

pub fn count(acid: char, seq: &str) -> usize {
	seq.chars().fold(0, |count, c| if c == acid { count + 1 } else { count })
}