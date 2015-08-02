extern crate rand;
use rand::distributions::{IndependentSample, Range};

pub struct Robot {
	name: String,
}

impl Robot {
	pub fn new() -> Robot {
		Robot { name: Self::generate_name() }
	}

	fn generate_name() -> String {

		let mut rng = rand::thread_rng();
		let numbers_range = Range::new(48u8, 57u8);
		let letters_range = Range::new(65u8, 90u8);

		let mut results = String::new();
		for _ in (0..3) { results.push(letters_range.ind_sample(&mut rng) as char); }
		for _ in (0..3) { results.push(numbers_range.ind_sample(&mut rng) as char); }

		results
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.name
	}

	pub fn reset_name(&mut self) {
		self.name = Self::generate_name();
	}
}