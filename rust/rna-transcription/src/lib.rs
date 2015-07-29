#[derive(PartialEq, Debug)]
pub struct RibonucleicAcid {
	pub sequence: String,
}

impl RibonucleicAcid {
	pub fn new(seq: &str) -> RibonucleicAcid {
		RibonucleicAcid { sequence: seq.to_string() }
	}
	pub fn as_ref(&self) -> &str { &self.sequence }
}

pub struct DeoxyribonucleicAcid<'a> {
	pub sequence: &'a str,
}

impl <'a>DeoxyribonucleicAcid<'a> {
	pub fn new(seq: &'a str) -> DeoxyribonucleicAcid<'a> {
		DeoxyribonucleicAcid { sequence: seq }
	}
	pub fn to_rna(&self) -> RibonucleicAcid {
		RibonucleicAcid::new(&self.transcription())
	}
	fn transcription(&self) -> String {
		self.sequence.chars().map(|c| {
			match c {
				'C' => 'G',
				'G' => 'C',
				'A' => 'U',
				'T' => 'A',
				_ => 'X'
			}
	  	}).collect::<String>()
	}
}