pub trait OccupationLoader {
	fn occupations() -> Vec<Occupation>;
}

pub trait InfoLoader {
	fn names() -> NameList;
	fn cities() -> Vec<String>;
}

pub trait SkillsLoader {
	fn skills_list() -> Vec<SkillData>;
}

#[derive(Debug)]
pub struct NameList {
	surnames: Vec<String>,
	male_names: Vec<String>,
	female_names: Vec<String>,
}

impl NameList {
	pub fn from(
		surnames: Vec<String>, 
		male_names: Vec<String>, 
		female_names: Vec<String>
	) -> Self {
		Self {
			surnames,
			male_names,
			female_names,
		}
	}

	pub fn surnames(&self) -> &Vec<String> {
        &self.surnames
    }

    pub fn male_names(&self) -> &Vec<String> {
        &self.male_names
    }

    pub fn female_names(&self) -> &Vec<String> {
        &self.female_names
    }
}

#[derive(Debug, Clone)]
pub enum SkillOption {
	Specific(String),
	AnyWithAmount(u8),
	OneOf(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Occupation {
	pub name: String,
	pub skills: Vec<SkillOption>,
	pub credit_rating: (u8, u8),
	pub skill_points_type: String,
}

#[derive(Debug, Clone)]
pub struct SkillData {
	pub name: String,
	pub value: u8,
	pub is_fighting_skill: bool,
}
