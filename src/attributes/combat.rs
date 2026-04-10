use crate::attributes::{DiceType, Characteristics, CharIndex::{Dex, Str, Siz}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Combat {
	move_rate: u8,
	damage_bonus: DamageBonus,
	build: i8,
}

impl Combat {
	pub fn create(age: u8, chars: &Characteristics) -> Self {
		let (damage_bonus, build) = damage_bonus_and_build(chars);
		Self {
			move_rate: move_rate(age, chars),
			damage_bonus,
			build,
		}
	}

	pub fn move_rate(&self) -> u8 {
		self.move_rate
	}

	pub fn damage_bonus(&self) -> DamageBonus {
		self.damage_bonus
	}

	pub fn build(&self) -> i8 {
		self.build
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DamageBonus {
	Int(i8),
	Dice(u8, DiceType),
}

fn move_rate(age: u8, chars: &Characteristics) -> u8 {
	let mov = if (chars[Dex] < chars[Siz]) && (chars[Str] < chars[Siz]) {
		7
	} else if (chars[Dex] > chars[Siz]) && (chars[Str] > chars[Siz]) {
		9
	} else {
		8
	};
	mov - match age {
		40..=49 => 1,
		50..=59 => 2,
		60..=69 => 3,
		70..=79 => 4,
		80..=89 => 5,
		_ => 0,
	}
}

fn damage_bonus_and_build(chars: &Characteristics) -> (DamageBonus, i8) {
	match chars[Str].as_u16() + chars[Siz].as_u16() {
		..=64 => (DamageBonus::Int(-2), -2),
		65..=84 => (DamageBonus::Int(-1), -1),
		85..=124 => (DamageBonus::Int(0), 0),
		125..=164 => (DamageBonus::Dice(1, DiceType::D4), 1),
		165..=204 => (DamageBonus::Dice(1, DiceType::D6), 1),
		205..=284 => (DamageBonus::Dice(2, DiceType::D6), 2),
		_ => unreachable!("Invalid sum of Strength and Size"),
	} 
}