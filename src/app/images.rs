// Stores single copy of picture that can be referenced elsewhere
pub struct Images {
	profile_picture: Option<egui::TextureHandle>,
}

impl Default for Images {
	
	fn default() -> Self {
		Self {
			profile_picture: Option::None,
		}
	}
}

impl Images {
	
	fn update(&mut self, ctx: &egui::Context, image: impl Into<egui::ImageData>) {

		if let Some(handle) = self.profile_picture.as_mut() {
			handle.set(
					image,
					Default::default()
			);
		} else {

			self.profile_picture = Some(ctx.load_texture(
				"Profile Picture",
				image,
				Default::default()
			));
		}
	}

	pub fn clone_profile_picture(&mut self, ctx: &egui::Context) -> Option<egui::TextureHandle>{
		if self.profile_picture.is_none() {
			self.update(ctx, egui::ColorImage::new([32, 32], egui::Color32::RED))
		}
		return self.profile_picture.clone()
	}

	pub fn load_image_from_raw(&mut self, ctx: &egui::Context, raw_file: Vec<u8>,) {

		let result = image::io::Reader::new(std::io::Cursor::new(raw_file))
			.with_guessed_format()
			.expect("Error loading iamge")
			// Convert to DyanmicImage and handle Result<>
			.decode();

		// Handle errors
		if let Ok(image) = result {
			let size = [image.width() as _, image.height() as _];
			let image_buffer = image.to_rgba8();
			let pixels = image_buffer.as_flat_samples();
	
			let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
			
			self.update(ctx, color_image);
		}
	
	}
}