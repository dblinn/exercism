#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Allergen {
	Eggs = 1,
	Peanuts = 2,
	Shellfish = 4,
	Strawberries = 8,
	Tomatoes = 16,
	Chocolate = 32,
	Pollen = 64,
	Cats = 128
}

pub struct Allergies(pub u32);

impl Allergies {
	pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
		let allergen_bits = *allergen as u32;
		(self.0 & allergen_bits) > 0
	}

	pub fn allergies(&self) -> Vec<Allergen> {
		[
			Allergen::Eggs,
			Allergen::Peanuts,
			Allergen::Shellfish,
			Allergen::Strawberries,
			Allergen::Tomatoes,
			Allergen::Chocolate,
			Allergen::Pollen,
			Allergen::Cats
		]
		.iter()
		.filter(|a| self.is_allergic_to(a))
		.map(|a| *a)
		.collect()
	}
}
