#[derive(serde::Serialize, serde::Deserialize)]
pub enum CurrentUser {
	LoggedIn(User),
	#[serde(skip)]
	Empty(NewUser),
}

pub struct NewUser {
	pub username: String,
	pub password: String,
}

impl Default for NewUser {
    fn default() -> Self {
        Self {
			username: String::new(),
			password: String::new()}
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
	pub username: String,
	password: String,
	is_logged_in: bool,

	#[serde(skip)]
	profile_image: super::images::OptionalImage,	
}

impl Default for User {
    fn default() -> Self {
        Self {
			username: String::from("Panda Test"),
			password: Default::default(),
			is_logged_in: false,
			profile_image: Default::default(),
		}
    }
}

impl User {
	pub fn new(username: String, password: String) -> Self {
		Self {
			username: username,
			password: password,
			is_logged_in: true,
			profile_image: Default::default(),
		}
	}
	#[allow(dead_code)]
	pub fn log_in(&mut self, password: String) -> bool {
		if password == self.password {
			self.is_logged_in = true;
			true
		} else {
			false
		}
	}
	#[allow(dead_code)]
	pub fn update_password(&mut self, old_password: String, new_password: String) {
		if old_password == self.password {
			self.password = new_password
		}
	}
	#[allow(dead_code)]
	pub fn is_user_logged_in(&self) -> bool {
		return self.is_logged_in
	}
	pub fn get_profile_picture(&mut self, ctx: &egui::Context) -> Option<(egui::TextureId, egui::Vec2)> {
		if let Some((id, size)) = self.profile_image.get_id_size(ctx) {
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