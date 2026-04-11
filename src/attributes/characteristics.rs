use crate::attributes::Skill;
use crate::random::Dice;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Characteristics([Skill; 8]);

impl Characteristics {
	pub fn create(age: u8, dice: &mut impl Dice, modifier: &mut impl CharsModifier) -> Self {
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

impl From<[Skill; 8]> for Characteristics {
	fn from(skills: [Skill; 8]) -> Self {
		Self(skills)
	}
}

impl Index<CharsIndex> for Characteristics {
	type Output = Skill;

	fn index(&self, idx: CharsIndex) -> &Self::Output {
		&self.0[idx.index()]
	}
}

impl IndexMut<CharsIndex> for Characteristics {
	fn index_mut(&mut self, idx: CharsIndex) -> &mut Self::Output {
		&mut self.0[idx.index()]
	}
}

pub trait CharsModifier {
	fn modify_by_age(&mut self, chars: &mut Characteristics, age: u8, dice: &mut impl Dice) {
		match age {
			..=19 => {
				self.deduct_from_str_or_size(5, chars);
				chars[CharsIndex::Edu] -= 5;
			},
			20..=39 => Self::edu_improvement_check(chars, dice),
			40..=49 => {
				self.deduct_from_str_con_or_dex(5, chars);
				chars[CharsIndex::App] -= 5;
				for _ in 0..2 {
					Self::edu_improvement_check(chars, dice);
				}
			},
			50..=59 => {
				self.deduct_from_str_con_or_dex(10, chars);
				chars[CharsIndex::App] -= 10;
				for _ in 0..3 {
					Self::edu_improvement_check(chars, dice);
				}
			},
			60..=69 => {
				self.deduct_from_str_con_or_dex(20, chars);
				chars[CharsIndex::App] -= 15;
				for _ in 0..4 {
					Self::edu_improvement_check(chars, dice);
				}
			},
			70..=79 => {
				self.deduct_from_str_con_or_dex(30, chars);
				chars[CharsIndex::App] -= 20;
				for _ in 0..4 {
					Self::edu_improvement_check(chars, dice);
				}
			},
			80.. => {
				self.deduct_from_str_con_or_dex(40, chars);
				chars[CharsIndex::App] -= 25;
				for _ in 0..4 {
					Self::edu_improvement_check(chars, dice);
				}
			}
		}
	}

	fn edu_improvement_check(chars: &mut Characteristics, dice: &mut impl Dice) {
		if dice.roll_100d() > chars[CharsIndex::Edu] {
			chars[CharsIndex::Edu] += dice.roll_10d();
		}
	}

	fn deduct_from_str_or_size(&mut self, points: u8, chars: &mut Characteristics);
	fn deduct_from_str_con_or_dex(&mut self, points: u8, chars: &mut Characteristics);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharsIndex {
	Str,
	Con,
	Siz,
	Dex,
	App,
	Int,
	Pow,
	Edu,
}

impl CharsIndex {
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
