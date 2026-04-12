#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Backstory {
	pub description: String,
	pub ideology: String,
	pub people: String,
	pub locations: String,
	pub possessions: String,
	pub traits: String,
	pub injuries: String,
	pub phobias: String,
	pub spells: String,
	pub encourters: String,
}

pub trait BackstoryBuilder {
	fn new() -> Self;
	fn description(self, description: &str) -> Self;
	fn ideology(self, ideology: &str) -> Self;
	fn people(self, ideology: &str) -> Self;
	fn locations(self, locations: &str) -> Self;
	fn possessions(self, possessions: &str) -> Self;
	fn traits(self, traits: &str) -> Self;
	fn injuries(self, injuries: &str) -> Self;
	fn phobias(self, phobias: &str) -> Self;
	fn spells(self, spells: &str) -> Self;
	fn encourters(self, encourters: &str) -> Self;
	fn build(&self) -> Backstory;
}
