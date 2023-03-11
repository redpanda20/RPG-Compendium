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

	character: Option<character::Character>,
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
			character: None
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
			character: None
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
		if !self.profile_image.is_some() {
			if let Some(image) = self.image_storage.clone() {
				self.update_profile_picture(ctx, image);
			}
		}
		return Some(self.profile_image.get(ctx))
	}
	pub fn update_profile_picture(&mut self, ctx: &egui::Context, raw_file: Vec<u8>) {
		if self.is_logged_in {
			self.profile_image.load_image_from_raw(ctx, raw_file.clone());
			self.image_storage = Some(raw_file.clone());
		}
	}
	pub fn get_character(&mut self) -> &mut Option<character::Character> {
		return &mut self.character
	}

	pub fn set_character(&mut self, name: String, racial_archetype: character::RacialArchetype) {
		self.character = Some(character::Character::new(name, racial_archetype));
	}

	pub fn remove_character(&mut self) {
		self.character = None
	}
}