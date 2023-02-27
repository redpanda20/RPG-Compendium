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
	white_fill: egui_extras::RetainedImage,
	black_fill: egui_extras::RetainedImage, 
}
impl StaticSvg {
	pub fn _empty() -> Self {
		let empty_svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="640" height="120">
		<style xmlns="" id="autoconsent-prehide"/></svg>"##;
		Self {
			white_fill: egui_extras::RetainedImage::from_svg_bytes("empty_white", empty_svg).unwrap(),
			black_fill: egui_extras::RetainedImage::from_svg_bytes("empty_black", empty_svg).unwrap(),
		}
	}

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
			white_fill: egui_extras::RetainedImage::from_svg_bytes_with_size(white_name, &white_bytes, ICON_SIZE).unwrap(),
			black_fill: egui_extras::RetainedImage::from_svg_bytes_with_size(black_name, &black_bytes, ICON_SIZE).unwrap(),
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
			white_fill: egui_extras::RetainedImage::from_svg_bytes_with_size(white_name, &white_bytes, icon_size).unwrap(),
			black_fill: egui_extras::RetainedImage::from_svg_bytes_with_size(black_name, &black_bytes, icon_size).unwrap(),
			};
	}

	pub fn new_precalculated(name: String, white_bytes: Vec<u8>, black_bytes: Vec<u8>) -> Self {
		let mut white_name = name.clone();
		white_name.push_str("_white");

		let mut black_name = name.clone();
		black_name.push_str("_black");

		return Self {
			white_fill: egui_extras::RetainedImage::from_svg_bytes(white_name, &white_bytes).unwrap(),
			black_fill: egui_extras::RetainedImage::from_svg_bytes(black_name, &black_bytes).unwrap(),
			};
	}

	pub fn get(&self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		if ctx.style().visuals.dark_mode {
			(self.white_fill.texture_id(ctx), self.white_fill.size_vec2())
		} else {
			(self.black_fill.texture_id(ctx), self.black_fill.size_vec2())
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