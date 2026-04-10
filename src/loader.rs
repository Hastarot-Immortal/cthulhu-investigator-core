pub trait Loader {
	fn cities() -> Vec<String>;
	fn names() -> NameList;
	fn occupations() -> Vec<Occupation>;
	fn skills_list() -> Vec<(String, u8)>;
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
	name: String,
	skills: Vec<SkillOption>,
	credit_rating: (u8, u8),
	skill_points_type: String,
}
