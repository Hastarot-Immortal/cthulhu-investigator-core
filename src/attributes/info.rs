#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Information {
	pub name: String,
	pub occupation: String,
	pub age: u8,
	pub sex: Sex,
	pub residence: String,
	pub birthplace: String,
}

impl Information {
	pub fn create(
		name: String, 
		occupation: String, 
		age: u8, 
		sex: Sex, 
		residence: String, 
		birthplace: String
	) -> Self {
		Self {
			name, 
			occupation, 
			age, 
			sex, 
			residence, 
			birthplace
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn occupation(&self) -> &String {
		&self.occupation
	}

	pub fn age(&self) -> u8 {
		self.age
	}

	pub fn sex(&self) -> Sex {
		self.sex
	}

	pub fn residence(&self) -> &String {
		&self.residence
	}

	pub fn birthplace(&self) -> &String {
		&self.birthplace
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sex {
	Male,
	Female,
	Other,
}
