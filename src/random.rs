pub trait Dice {
	fn roll_6d(&mut self) -> u8;
	fn roll_6d_with_amount(&mut self, amount: u8) -> u8;
	fn roll_100d(&mut self) -> u8;
}