pub struct CodonNames {
	pairs: Vec<(&'static str, &'static str)>,
}

impl CodonNames {
	fn expand(&self, code: &str) -> String {
		let expansion = |c| match c {
			'N' => 'T',
			'M' => 'C',
			'R' => 'A',
			'Y' => 'T',
			'H' => 'T',
			_ => c,
		};
		code.chars().map(|c| expansion(c)).collect::<String>()
	}

	pub fn name_for(&self, codon: &str) -> Result<&'static str, &'static str> {
		for &(code, name) in &self.pairs {
			if code == self.expand(codon) {
				return Ok(name);
			}
		}
		Err("")
	}
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> CodonNames {
	CodonNames { pairs: pairs }
}