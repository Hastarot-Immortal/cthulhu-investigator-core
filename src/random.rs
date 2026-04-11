use crate::attributes::{
	Characteristics, 
	CharIndex::{App, Edu}, 
	Information, 
	Sex
};
use crate::loader::Loader;

pub trait FromSeed {
	type Seed;
	fn from_seed(seed: Self::Seed) -> Self;
}

pub trait Dice: FromSeed {
	fn roll_6d(&mut self) -> u8;
	fn roll_6d_with_amount(&mut self, amount: u8) -> u8;
	fn roll_10d(&mut self) -> u8;
	fn roll_100d(&mut self) -> u8;
}

pub trait CharacteristicsModifier: FromSeed {
	fn modify_by_age(&mut self, chars: &mut Characteristics, age: u8, dice: &mut impl Dice) {
		match age {
			..=19 => {
				self.deduct_from_str_or_size(5, chars);
				chars[Edu] -= 5;
			},
			20..=39 => Self::edu_improvement_check(chars, dice),
			40..=49 => {
				self.deduct_from_str_con_or_dex(5, chars);
				chars[App] -= 5;
				for _ in 0..2 {
					Self::edu_improvement_check(chars, dice);
				}
			},
			50..=59 => {
				self.deduct_from_str_con_or_dex(10, chars);
				chars[App] -= 10;
				for _ in 0..3 {
					Self::edu_improvement_check(chars, dice);
				}
			},
			60..=69 => {
				self.deduct_from_str_con_or_dex(20, chars);
				chars[App] -= 15;
				for _ in 0..4 {
					Self::edu_improvement_check(chars, dice);
				}
			},
			70..=79 => {
				self.deduct_from_str_con_or_dex(30, chars);
				chars[App] -= 20;
				for _ in 0..4 {
					Self::edu_improvement_check(chars, dice);
				}
			},
			80.. => {
				self.deduct_from_str_con_or_dex(40, chars);
				chars[App] -= 25;
				for _ in 0..4 {
					Self::edu_improvement_check(chars, dice);
				}
			}
		}
	}

	fn edu_improvement_check(chars: &mut Characteristics, dice: &mut impl Dice) {
		if dice.roll_100d() > chars[Edu] {
			chars[Edu] += dice.roll_10d();
		}
	}

	fn deduct_from_str_or_size(&mut self, points: u8, chars: &mut Characteristics);
	fn deduct_from_str_con_or_dex(&mut self, points: u8, chars: &mut Characteristics);
}

pub trait InfoBuilder: FromSeed {
	fn loader(self, loader: impl Loader) -> Self;
	fn name(self, name: &str) -> Self;
	fn occupation(self, occupation: &str) -> Self;
	fn age(self, age: u8) -> Self;
	fn sex(self, sex: Sex) -> Self;
	fn residence(self, residence: &str) -> Self;
	fn birthplace(self, birthplace: &str) -> Self;
	fn build(&mut self) -> Information;
}
