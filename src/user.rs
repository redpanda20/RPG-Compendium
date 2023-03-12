use crate::resources::image;
use crate::mystward::character;

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
	is_mutable: bool,
	is_logged_in: bool,

	pub username: String,
	password: String,

	#[serde(skip)]
	profile_image: image::OptionalImage,	
	image_storage: Option<Vec<u8>>,

	characters: Vec<character::Character>,
	active_character: Option<character::Character>,
}

impl Default for User {
    fn default() -> Self {
        Self {
			is_mutable: false,
			is_logged_in: false,
			username: Default::default(),
			password: Default::default(),
			profile_image: Default::default(),
			image_storage: None,
            characters: Vec::new(),
            active_character: None,
		}
    }
}

impl User {
	pub fn new(username: String, password: String) -> Self {
		Self {
			username,
			password,
			is_mutable: true,
			is_logged_in: true,
			profile_image: Default::default(),
			image_storage: None,
            characters: Vec::new(),
            active_character: None,
		}
	}

	#[allow(dead_code)]
	pub fn log_in(&mut self, password: String) -> bool {
		if self.is_mutable && password == self.password {
			self.is_logged_in = true;
			true
		} else {
			false
		}
	}
	pub fn log_out(&mut self) {
		if self.is_mutable {
			self.is_logged_in = false
		}
	}
	#[allow(dead_code)]
	pub fn update_password(&mut self, old_password: String, new_password: String) {
		if old_password == self.password {
			self.password = new_password
		}
	}
	pub fn is_logged_in(&self) -> bool {
		return self.is_logged_in
	}
	pub fn is_mutable(&self) -> bool {
		return self.is_mutable
	}
	pub fn get_profile_picture(&mut self, ctx: &egui::Context) -> Option<(egui::TextureId, egui::Vec2)> {
		if !self.is_logged_in {
			return None
		}
		if !self.profile_image.is_set() {
			if let Some(image) = self.image_storage.clone() {
				self.update_profile_picture(ctx, image);
			}
		}
		return Some(self.profile_image.get(ctx))
	}
	pub fn update_profile_picture(&mut self, ctx: &egui::Context, raw_file: Vec<u8>) {
		if self.is_logged_in {
			self.profile_image.update(ctx, raw_file.clone());
			self.image_storage = Some(raw_file.clone());
		}
	}

	pub fn get_all_characters(&mut self) -> &mut Vec<character::Character> {
		return &mut self.characters;
	}

	pub fn get_active_character(&mut self) -> &mut Option<character::Character> {
		return &mut self.active_character
	}

	pub fn remove_active_character(&mut self) {
		if let Some(character) = &self.active_character {
			self.characters.retain(|char| char.character_info.name != character.character_info.name);
		}
		self.active_character = None;
	}

	pub fn set_active_character(&mut self, new_character: character::Character) {
		if let Some(character) = &self.active_character {
			match self.characters
				.iter()
				.position(|ref char| char.character_info.name == character.character_info.name)
			{
				Some(index) => self.characters[index] = character.clone(),
				None => self.characters.push(character.clone()),
			}
		}
		self.characters.retain(|char| &char.character_info.name != &new_character.character_info.name);
		self.active_character = Some(new_character);
	}

	pub fn add_character(&mut self, name: String, racial_archetype: character::RacialArchetype) {
		let new_character = character::Character::new(name, racial_archetype);
		self.set_active_character(new_character);
	}

}