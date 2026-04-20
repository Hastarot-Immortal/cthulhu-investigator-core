use std::ops::RangeBounds;

/// A trait for creating objects from seed. 
pub trait FromSeed {
	type Seed;
	fn from_seed(seed: Self::Seed) -> Self;
}

/// A trait for imitating a dice (basically a (pseudo)random generator for [`u8`])
pub trait Dice: FromSeed {
	/// Generates a random value in a given range
	fn roll_range<R: RangeBounds<u8>>(&mut self, range: R) -> u8;

	/// Generates a random value between `1` and `6` inclusively
	fn roll_6d(&mut self) -> u8 {
		self.roll_range(1..=6)
	}

	/// Generates a random value between `amount` and `6 * amount` inclusively. If `amount` is too big, then it returns [`u8::MAX`].
	fn roll_6d_with_amount(&mut self, amount: u8) -> u8 {
		(0..amount)
		.map(|_: u8| self.roll_6d() )
		.fold(0, |d1, d2| d1.saturating_add(d2))
	}

	/// Generates a random value between `1` and `10` inclusively.
	fn roll_10d(&mut self) -> u8 {
		self.roll_range(1..=10)
	}

	/// Generates a random value between `1` and `100` inclusively.
	fn roll_100d(&mut self) -> u8 {
		self.roll_range(1..=100)
	}
}
