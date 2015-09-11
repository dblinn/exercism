extern crate rand;

pub struct Robot {
	name: String,
}

impl Robot {
	pub fn new() -> Robot {
		Robot { name: Self::generate_name() }
	}

	fn generate_name() -> String {
		let mut rng = rand::thread_rng();
		rand::sample(&mut rng, b'A'..b'Z', 3).iter().map(|c| *c as char)
			.chain(rand::sample(&mut rng, b'0'..b'9', 3).iter().map(|c| *c as char))
			.collect::<String>()
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.name
	}

	pub fn reset_name(&mut self) {
		self.name = Self::generate_name();
	}
}
