use crate::attributes::{
	DiceType, 
	Characteristics, 
	CharsIndex::{Dex, Str, Siz}
};

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
	let (dex, st, siz) = (chars[Dex], chars[Str], chars[Siz]);

	let mov = if (dex < siz) && (st < siz) {
		7
	} else if (dex > siz) && (st > siz) {
		9
	} else {
		8
	};

	mov - match age {
		40..=49 => 1,
		50..=59 => 2,
		60..=69 => 3,
		70..=79 => 4,
		80.. => 5,
		_ => 0,
	}
}

fn damage_bonus_and_build(chars: &Characteristics) -> (DamageBonus, i8) {
	match chars[Str].as_u16() + chars[Siz].as_u16() {
		..=64 => (DamageBonus::Int(-2), -2),
		65..=84 => (DamageBonus::Int(-1), -1),
		85..=124 => (DamageBonus::Int(0), 0),
		125..=164 => (DamageBonus::Dice(1, DiceType::D4), 1),
		165..=204 => (DamageBonus::Dice(1, DiceType::D6), 2),
		205..=284 => (DamageBonus::Dice(2, DiceType::D6), 3),
		285..=364 => (DamageBonus::Dice(3, DiceType::D6), 4),
		365..=444 => (DamageBonus::Dice(4, DiceType::D6), 5),
		445.. => (DamageBonus::Dice(5, DiceType::D6), 6),
	}
}
