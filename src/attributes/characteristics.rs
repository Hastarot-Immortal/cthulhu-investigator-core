use crate::attributes::Skill;
use crate::random::{Dice, CharacteristicsModifier};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Characteristics([Skill; 8]);

impl Characteristics {
	pub fn create(age: u8, dice: &mut impl Dice, modifier: &mut impl CharacteristicsModifier) -> Self {
		let mut chars = Self([
			Skill::from(dice.roll_6d_with_amount(3) * 5),
			Skill::from(dice.roll_6d_with_amount(3) * 5),
			Skill::from((dice.roll_6d_with_amount(2) + 6) * 5),
			Skill::from(dice.roll_6d_with_amount(3) * 5),
			Skill::from(dice.roll_6d_with_amount(3) * 5),
			Skill::from((dice.roll_6d_with_amount(2) + 6) * 5),
			Skill::from(dice.roll_6d_with_amount(3) * 5),
			Skill::from((dice.roll_6d_with_amount(2) + 6) * 5),
		]);
		modifier.modify_by_age(&mut chars, age, dice);
		chars
	}
}

impl Index<CharIndex> for Characteristics {
	type Output = Skill;

	fn index(&self, idx: CharIndex) -> &Self::Output {
		&self.0[idx.index()]
	}
}

impl IndexMut<CharIndex> for Characteristics {
	fn index_mut(&mut self, idx: CharIndex) -> &mut Self::Output {
		&mut self.0[idx.index()]
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharIndex {
	Str,
	Con,
	Siz,
	Dex,
	App,
	Int,
	Pow,
	Edu,
}

impl CharIndex {
	fn index(&self) -> usize {
		match *self {
			Self::Str => 0,
			Self::Con => 1,
			Self::Siz => 2,
			Self::Dex => 3,
			Self::App => 4,
			Self::Int => 5,
			Self::Pow => 6,
			Self::Edu => 7,
		}
	}
}