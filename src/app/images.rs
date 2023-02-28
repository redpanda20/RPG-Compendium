// Stores single copy of picture that can be referenced elsewhere
#[derive(Clone)]
pub struct OptionalImage {
	pub profile_picture: Option<egui::TextureHandle>,
}
impl Default for OptionalImage {
	
	fn default() -> Self {
		Self {
			profile_picture: None,
		}
	}
}
impl OptionalImage {
	
	fn update(&mut self, ctx: &egui::Context, image: impl Into<egui::ImageData>) {

		let Some(handle) = self.profile_picture.as_mut() else {
			self.profile_picture = Some(ctx.load_texture(
				"Profile picture",
				image,
				Default::default()
			));
			return;
		};
		handle.set(
			image,
			Default::default()
		);
	}

	pub fn get(&mut self) -> Option<(egui::TextureId, egui::Vec2)> {
		let Some(profile) = self.profile_picture.clone() else {
			return None
		};
		return Some((profile.id(), profile.size_vec2()))
	}

	pub fn _get_picture(&mut self, ctx: &egui::Context) -> egui::TextureHandle {
		if self.profile_picture.is_none() {
			self.update(ctx, egui::ColorImage::new([32, 32], egui::Color32::RED))

		}
		self.profile_picture.clone().unwrap()
	}

	pub fn load_image_from_raw(&mut self, ctx: &egui::Context, raw_file: Vec<u8>) {

		let result = image::io::Reader::new(std::io::Cursor::new(raw_file))
			.with_guessed_format()
			.expect("Error loading iamge")
			// Convert to DyanmicImage and handle Result<>
			.decode();

		// Handle errors
		let Ok(image) = result else {
			return;
		};
		let size = [image.width() as _, image.height() as _];
		let image_buffer = image.to_rgba8();
		let pixels = image_buffer.as_flat_samples();

		let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
		
		self.update(ctx, color_image);
	}
}

pub struct StaticSvg {
	base_image: egui_extras::RetainedImage,
	alt_image: Option<egui_extras::RetainedImage>, 
}
impl StaticSvg {
	pub fn new(name: String, img_bytes: Vec<u8>) -> Self {
		const ICON_SIZE: egui_extras::image::FitTo = egui_extras::image::FitTo::Height(24);

		// Get position of first '>'
		let (index, _) = img_bytes.clone().iter().enumerate().find(|(_, b)| **b == b'>').unwrap();
		// Split into left and right at position
		let (left, right) = img_bytes.split_at(index);
		let white_bytes = [left, br##" fill="#FFFFFF""##, right].concat();
		
		let mut white_name = name.clone();
		white_name.push_str("_black");

		// Calculate white image
		let black_bytes = img_bytes.clone();
		let mut black_name = name.clone();
		black_name.push_str("_white");

		return Self {
			base_image: egui_extras::RetainedImage::from_svg_bytes_with_size(white_name, &white_bytes, ICON_SIZE).unwrap(),
			alt_image: Some(egui_extras::RetainedImage::from_svg_bytes_with_size(black_name, &black_bytes, ICON_SIZE).unwrap()),
			};
	}

	pub fn new_with_size(name: String, img_bytes: Vec<u8>, size: [u32; 2]) -> Self {
		let icon_size = egui_extras::image::FitTo::Size(size[0], size[1]);
		// Get position of first '>'
		let (index, _) = img_bytes.clone().iter().enumerate().find(|(_, b)| **b == b'>').unwrap();
		// Split into left and right at position
		let (left, right) = img_bytes.split_at(index);
		let white_bytes = [left, br##" fill="#FFFFFF""##, right].concat();
		
		let mut white_name = name.clone();
		white_name.push_str("_black");

		// Calculate white image
		let black_bytes = img_bytes.clone();
		let mut black_name = name.clone();
		black_name.push_str("_white");

		return Self {
			base_image: egui_extras::RetainedImage::from_svg_bytes_with_size(white_name, &white_bytes, icon_size).unwrap(),
			alt_image: Some(egui_extras::RetainedImage::from_svg_bytes_with_size(black_name, &black_bytes, icon_size).unwrap()),
			};
	}

	pub fn new_precalculated(name: String, white_bytes: Vec<u8>, black_bytes: Vec<u8>) -> Self {
		let mut white_name = name.clone();
		white_name.push_str("_white");

		let mut black_name = name.clone();
		black_name.push_str("_black");

		return Self {
			base_image: egui_extras::RetainedImage::from_svg_bytes(white_name, &white_bytes).unwrap(),
			alt_image: Some(egui_extras::RetainedImage::from_svg_bytes(black_name, &black_bytes).unwrap()),
			};
	}

	pub fn new_single(name: String, img_bytes: Vec<u8>) -> Self {
		return Self {
			base_image: egui_extras::RetainedImage::from_svg_bytes(name, &img_bytes).unwrap(),
			alt_image: None }
	}

	pub fn get(&self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		let Some(alt) = self.alt_image.as_ref() else {
			return (self.base_image.texture_id(ctx), self.base_image.size_vec2())
		};
		if ctx.style().visuals.dark_mode {
			(self.base_image.texture_id(ctx), self.base_image.size_vec2())
		} else {
			(alt.texture_id(ctx), alt.size_vec2())
		}
	}
}
pub const NO_IMAGE: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px"><path d="M0 0h24v24H0z" fill="none"/><path d="M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z"/></svg>"##;
pub const ACCOUNT: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px"><g><rect fill="none" height="24" width="24"/></g><g><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 4c1.93 0 3.5 1.57 3.5 3.5S13.93 13 12 13s-3.5-1.57-3.5-3.5S10.07 6 12 6zm0 14c-2.03 0-4.43-.82-6.14-2.88C7.55 15.8 9.68 15 12 15s4.45.8 6.14 2.12C16.43 19.18 14.03 20 12 20z"/></g></svg>"##;
pub const _PERSON_OFF: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><rect fill="none" height="24" width="24"/><path d="M20,17.17l-3.37-3.38c0.64,0.22,1.23,0.48,1.77,0.76C19.37,15.06,19.98,16.07,20,17.17z M21.19,21.19l-1.41,1.41L17.17,20H4 v-2.78c0-1.12,0.61-2.15,1.61-2.66c1.29-0.66,2.87-1.22,4.67-1.45L1.39,4.22l1.41-1.41L21.19,21.19z M15.17,18l-3-3 c-0.06,0-0.11,0-0.17,0c-2.37,0-4.29,0.73-5.48,1.34C6.2,16.5,6,16.84,6,17.22V18H15.17z M12,6c1.1,0,2,0.9,2,2 c0,0.86-0.54,1.59-1.3,1.87l1.48,1.48C15.28,10.64,16,9.4,16,8c0-2.21-1.79-4-4-4c-1.4,0-2.64,0.72-3.35,1.82l1.48,1.48 C10.41,6.54,11.14,6,12,6z"/></svg>"##;
// pub const NO_PROFILE: &[u8] =
// br##"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px"><rect fill="none" height="24" width="24"/><path d="M20,17.17l-3.37-3.38c0.64,0.22,1.23,0.48,1.77,0.76C19.37,15.06,19.98,16.07,20,17.17z M21.19,21.19l-1.41,1.41L17.17,20H4 v-2.78c0-1.12,0.61-2.15,1.61-2.66c1.29-0.66,2.87-1.22,4.67-1.45L1.39,4.22l1.41-1.41L21.19,21.19z M15.17,18l-3-3 c-0.06,0-0.11,0-0.17,0c-2.37,0-4.29,0.73-5.48,1.34C6.2,16.5,6,16.84,6,17.22V18H15.17z M12,6c1.1,0,2,0.9,2,2 c0,0.86-0.54,1.59-1.3,1.87l1.48,1.48C15.28,10.64,16,9.4,16,8c0-2.21-1.79-4-4-4c-1.4,0-2.64,0.72-3.35,1.82l1.48,1.48 C10.41,6.54,11.14,6,12,6z"/></svg>"##;
pub const TOGGLE_OFF: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><path d="M0 0h24v24H0z" fill="none"/><path d="M17 7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h10c2.76 0 5-2.24 5-5s-2.24-5-5-5zM7 15c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z"/></svg>"##;
pub const TOGGLE_ON: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0z" fill="none"/><path d="M17 7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h10c2.76 0 5-2.24 5-5s-2.24-5-5-5zm0 8c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z"/></svg>"##;
pub const MORE_VERT: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/></svg>"##;
pub const HOME: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px"><path d="M0 0h24v24H0z" fill="none"/><path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/></svg>"##;
pub const BOOKLET: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px"><path d="M0 0h24v24H0z" fill="none"/><path d="M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z"/></svg>"##;

pub const IGNITION: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#FF8080"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M12,12.9l-2.13,2.09C9.31,15.55,9,16.28,9,17.06C9,18.68,10.35,20,12,20s3-1.32,3-2.94c0-0.78-0.31-1.52-0.87-2.07 L12,12.9z"/></g><g><path d="M16,6l-0.44,0.55C14.38,8.02,12,7.19,12,5.3V2c0,0-8,4-8,11c0,2.92,1.56,5.47,3.89,6.86C7.33,19.07,7,18.1,7,17.06 c0-1.32,0.52-2.56,1.47-3.5L12,10.1l3.53,3.47c0.95,0.93,1.47,2.17,1.47,3.5c0,1.02-0.31,1.96-0.85,2.75 c1.89-1.15,3.29-3.06,3.71-5.3C20.52,10.97,18.79,7.62,16,6z"/></g></g></g></svg>"##;
pub const LIFE: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#80FF80"><path d="M0 0h24v24H0z" fill="none"/><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>"##;
pub const DESIGN: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#D8D825"><g><rect fill="none" height="24" width="24"/></g><g><g><g><polygon points="12.16,3 11.84,3 9.21,8.25 14.79,8.25"/></g><g><polygon points="16.46,8.25 21.62,8.25 19,3 13.84,3"/></g><g><polygon points="21.38,9.75 12.75,9.75 12.75,20.1"/></g><g><polygon points="11.25,20.1 11.25,9.75 2.62,9.75"/></g><g><polygon points="7.54,8.25 10.16,3 5,3 2.38,8.25"/></g></g></g></svg>"##;
pub const ASTRAL: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#3867a5"><path d="M0 0h24v24H0z" fill="none"/><path d="M19 9l1.25-2.75L23 5l-2.75-1.25L19 1l-1.25 2.75L15 5l2.75 1.25L19 9zm-7.5.5L9 4 6.5 9.5 1 12l5.5 2.5L9 20l2.5-5.5L17 12l-5.5-2.5zM19 15l-1.25 2.75L15 19l2.75 1.25L19 23l1.25-2.75L23 19l-2.75-1.25L19 15z"/></svg>"##;
pub const FORCE: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#808080"><g><rect fill="none" height="24" width="24"/></g><g><g><polygon points="15.5,5 11,5 16,12 11,19 15.5,19 20.5,12"/><polygon points="8.5,5 4,5 9,12 4,19 8.5,19 13.5,12"/></g></g></svg>"##;
pub const WISDOM: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#bf819e"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M9,4v1.38c-0.83-0.33-1.72-0.5-2.61-0.5c-1.79,0-3.58,0.68-4.95,2.05l3.33,3.33h1.11v1.11c0.86,0.86,1.98,1.31,3.11,1.36 V15H6v3c0,1.1,0.9,2,2,2h10c1.66,0,3-1.34,3-3V4H9z M7.89,10.41V8.26H5.61L4.57,7.22C5.14,7,5.76,6.88,6.39,6.88 c1.34,0,2.59,0.52,3.54,1.46l1.41,1.41l-0.2,0.2c-0.51,0.51-1.19,0.8-1.92,0.8C8.75,10.75,8.29,10.63,7.89,10.41z M19,17 c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2h-6v-2.59c0.57-0.23,1.1-0.57,1.56-1.03l0.2-0.2L15.59,14H17v-1.41l-6-5.97V6h8V17z"/></g></g></svg>"##;
pub const ENTROPY: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0z" fill="none"/><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2z"/></svg>"##;

pub const NEG_ONE: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="48" viewBox="0 96 960 960" width="48"><path d="M400 646H120v-60h280v60Zm250 210V389l-99 71-35-53 153-111h51v560h-70Z"/></svg>"##;
pub const NEG_TWO: &[u8] =
br##"<svg xmlns="http://www.w3.org/2000/svg" height="48" viewBox="0 96 960 960" width="48"><path d="M484 856v-70l207-211q34-35 49.5-64t15.5-60q0-42-25-66.5T662 360q-38 0-66 18.5T556 429l-62-25q20-50 65.5-79T662 296q71 0 115.5 43T822 451q0 41-19 79t-64 83L563 793l2 3h275v60H484Zm-84-210H120v-60h280v60Z"/></svg>"##;