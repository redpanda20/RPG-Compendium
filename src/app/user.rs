#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
	is_mutable: bool,
	is_logged_in: bool,

	pub username: String,
	password: String,

	#[serde(skip)]
	profile_image: super::images::OptionalImage,	
}

impl Default for User {
    fn default() -> Self {
        Self {
			is_mutable: false,
			is_logged_in: false,
			username: Default::default(),
			password: Default::default(),
			profile_image: Default::default(),
		}
    }
}

impl User {
	pub fn new(username: String, password: String) -> Self {
		Self {
			username: username,
			password: password,
			is_mutable: true,
			is_logged_in: true,
			profile_image: Default::default(),
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
	pub fn get_profile_picture(&mut self) -> Option<(egui::TextureId, egui::Vec2)> {
		if let Some((id, size)) = self.profile_image.get() {
			Some((id, size))
		} else {
			None
		}
	}
	pub fn update_profile_picture(&mut self, ctx: &egui::Context, raw_file: Vec<u8>) {
		if self.is_logged_in {
			self.profile_image.load_image_from_raw(ctx, raw_file);
		}
	}
}