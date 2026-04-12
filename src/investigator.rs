use crate::attributes::*;
use crate::utils::FastMap;
use std::hash::Hash;

pub struct Investigator<S: Hash + Eq, E: Eq> {
	pub info: Information,
	pub characteristics: Characteristics,
	pub combat: Combat,
	pub points: Points,
	pub skills: FastMap<S, Skill>,
	pub backstory: Backstory,
	pub cash_assets: CashAssets,
	pub equipment: Vec<E>,
}
