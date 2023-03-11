// Stores single copy of picture that can be referenced elsewhere

fn image_to_texture(image: Vec<u8>) -> egui::ColorImage {
	let result = image::io::Reader::new(std::io::Cursor::new(image))
		.with_guessed_format()
		.expect("Error loading iamge")
		.decode();

	// Handle errors
	let Ok(image) = result else {
		return egui::ColorImage::new([30, 30], egui::Color32::RED);
	};
	let size = [image.width() as _, image.height() as _];
	let image_buffer = image.to_rgba8();
	let pixels = image_buffer.as_flat_samples();

	return egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
}

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

	pub fn load_image_from_raw(&mut self, ctx: &egui::Context, raw_file: Vec<u8>) {
		self.update(
			ctx,
			image_to_texture(raw_file)
		);
	}

	pub fn is_some(&self) -> bool {
		return self.profile_picture.is_some()
	}

	pub fn get(&self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		let image = if let Some(profile) = self.profile_picture.clone() {
			profile
		} else {
			let image = match ctx.style().visuals.dark_mode {
				true => super::defines::SELECT_IMAGE_LIGHT.to_vec(),
				false => super::defines::SELECT_IMAGE.to_vec()
			};
			ctx.load_texture("No Image", image_to_texture(image), Default::default())
		};
		return (image.id(), image.size_vec2())
	}
}