use crate::attributes::{Characteristics, CharsIndex::{Siz, Con, Pow}};
use crate::random::Dice;

pub struct Points {
	pub hp: u8,
	pub magic: u8,
	pub sanity: u8,
	pub luck: u8, 
}

impl Points {
	pub fn create(chars: &Characteristics, age: u8, dice: &mut impl Dice) -> Points {
		Self {
			hp: (chars[Siz] + chars[Con]).as_u8() / 10,
			sanity: chars[Pow].as_u8(),
			magic: chars[Pow].as_u8() / 5,
			luck: (dice.roll_6d_with_amount(3) * 5).max(match age {
				..=19 => dice.roll_6d_with_amount(3) * 5,
				_ => 0,	
			})
		}
	}
}
