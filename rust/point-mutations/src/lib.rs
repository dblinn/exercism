pub fn hamming_distance(s1: &str, s2: &str) -> usize {
	s1.chars().zip(s2.chars()).fold(0, |accum, (x, y)| {
		if x == y { accum } else { accum + 1}
	})
}