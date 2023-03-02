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

	pub fn is_some(&self) -> bool {
		return self.profile_picture.is_some()
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