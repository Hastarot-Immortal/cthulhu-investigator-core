pub mod skill;
pub mod characteristics;
pub mod points;
pub mod info;
pub mod combat;

pub use skill::Skill;
pub use characteristics::{Characteristics, CharIndex};
pub use points::Points;
pub use info::{Information, Sex};
pub use combat::Combat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiceType {
	D4,
	D6,
}
