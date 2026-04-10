use std::ops;
use std::cmp::{PartialEq, PartialOrd, Ordering};

fn validate_skill(value: u8) -> u8 {
	value.min(99) 
}

fn sub_skill(first: u8, second: u8) -> u8 {
	first.saturating_sub(second).max(5)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Skill(u8);

impl From<u8> for Skill {
	fn from(value: u8) -> Self {
		Self(validate_skill(value))
	}
}

impl ops::Add<u8> for Skill {
	type Output = Self;

	fn add(self, rhs: u8) -> Self::Output {
		Self(validate_skill(self.0 + rhs))
	}
}

impl ops::AddAssign<u8> for Skill {
	fn add_assign(&mut self, rhs: u8) {
		self.0 = validate_skill(self.0 + rhs)
	}
}

impl ops::Sub<u8> for Skill {
	type Output = Self;

	fn sub(self, rhs: u8) -> Self::Output {
		Self(sub_skill(self.0, rhs))
	}
}

impl ops::SubAssign<u8> for Skill {
	fn sub_assign(&mut self, rhs: u8) {
		self.0 = sub_skill(self.0, rhs); 
	}
}

impl PartialEq<u8> for Skill {
	fn eq(&self, other: &u8) -> bool {
		self.0 == *other
	}
}

impl PartialOrd<u8> for Skill {
	fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
		Some(self.0.cmp(other))
	}
}

impl PartialEq<Skill> for u8 {
	fn eq(&self, other: &Skill) -> bool {
		*self == other.0
	}
}

impl PartialOrd<Skill> for u8 {
	fn partial_cmp(&self, other: &Skill) -> Option<Ordering> {
		Some(self.cmp(&other.0))
	}
}