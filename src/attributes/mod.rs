pub mod skill;
pub mod characteristics;
pub mod points;

pub use skill::Skill;
pub use characteristics::{Characteristics, CharIndex};
pub use points::Points;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Information {
	pub name: String,
	pub occupation: String,
	pub age: u8,
	pub sex: Sex,
	pub residence: String,
	pub birthplace: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sex {
	Male,
	Female,
	Other,
}
