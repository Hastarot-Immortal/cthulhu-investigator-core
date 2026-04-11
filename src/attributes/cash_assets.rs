#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CashAssets {
	pub cash: f32,
	pub assets: u64,
	pub spending_level: f32,
}

pub trait CashAssetsBuilder {
	fn build(credit_rating: u8) -> CashAssets;
}

pub struct Old1920sCABuilder;

impl CashAssetsBuilder for Old1920sCABuilder {
	fn build(credit_rating: u8) -> CashAssets {
		let c_r_f32 = credit_rating as f32;
		let c_r_u64 = credit_rating as u64;
		let (cash, assets, spending_level) = match credit_rating {
			0 => (0.5, 0, 0.5),
			1..=9 => (c_r_f32, c_r_u64 * 10, 2.0),
			10..=49 => (c_r_f32 * 2.0, c_r_u64 * 50, 10.0),
			50..=89 => (c_r_f32 * 5.0, c_r_u64 * 500, 50.0),
			90..=98 => (c_r_f32 * 20.0, c_r_u64 * 2000, 250.0),
			99.. => (50_000.0, 5_000_000, 5_000.0)
		};
		CashAssets{cash, assets, spending_level}
	}
}

pub struct ModernCABuilder;

impl CashAssetsBuilder for ModernCABuilder {
	fn build(credit_rating: u8) -> CashAssets {
		let c_r_f32 = credit_rating as f32;
		let c_r_u64 = credit_rating as u64;
		let (cash, assets, spending_level) = match credit_rating {
			0 => (10.0, 0, 10.0),
			1..=9 => (c_r_f32 * 20.0, c_r_u64 * 200, 40.0),
			10..=49 => (c_r_f32 * 40.0, c_r_u64 * 1000, 200.0),
			50..=89 => (c_r_f32 * 100.0, c_r_u64 * 10_000, 1000.0),
			90..=98 => (c_r_f32 * 400.0, c_r_u64 * 40_000, 5000.0),
			99.. => (1_000_000.0, 100_000_000, 100_000.0),
		};
		CashAssets{cash, assets, spending_level}
	}
}
