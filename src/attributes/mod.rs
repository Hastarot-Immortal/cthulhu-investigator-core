pub mod skill;
pub mod characteristics;
pub mod points;
pub mod info;
pub mod combat;
pub mod cash_assets;
pub mod backstory;

pub use skill::Skill;
pub use characteristics::{Characteristics, CharsIndex, CharsModifier};
pub use points::Points;
pub use info::{Information, Sex, InfoBuilder};
pub use combat::{Combat, DamageBonus};
pub use cash_assets::{CashAssets, CashAssetsBuilder, 
	Old1920sCABuilder, ModernCABuilder
};
pub use backstory::{Backstory, BackstoryBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiceType {
	D4,
	D6,
}
