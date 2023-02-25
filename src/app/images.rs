// Stores single copy of picture that can be referenced elsewhere
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

		if let Some(handle) = self.profile_picture.as_mut() {
			handle.set(
					image,
					Default::default());
		} else {
			self.profile_picture = Some(ctx.load_texture(
				"Profile Picture",
				image,
				Default::default()));
		}
	}

	pub fn get_id_size(&mut self, ctx: &egui::Context) -> Option<(egui::TextureId, egui::Vec2)> {
		if let Some(profile) = self.profile_picture.clone() {
			Some((profile.id(), profile.size_vec2()))
		} else {
			None
		}
	}

	pub fn get_picture(&mut self, ctx: &egui::Context) -> egui::TextureHandle {
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
		if let Ok(image) = result {
			let size = [image.width() as _, image.height() as _];
			let image_buffer = image.to_rgba8();
			let pixels = image_buffer.as_flat_samples();
	
			let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
			
			self.update(ctx, color_image);
		}
	
	}
}

pub struct StaticSvg {
	white_fill: egui_extras::RetainedImage,
	black_fill: egui_extras::RetainedImage, 
}
impl StaticSvg {
	pub fn new(name: String, img_bytes: Vec<u8>) -> Self {

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
			white_fill: egui_extras::RetainedImage::from_svg_bytes(white_name, &white_bytes).unwrap(),
			black_fill: egui_extras::RetainedImage::from_svg_bytes(black_name, &black_bytes).unwrap(),
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


	pub fn show(&self, ui: &mut egui::Ui, is_dark_mode: bool) -> egui::Response {
		if is_dark_mode {
			self.white_fill.show(ui)
		} else {
			self.black_fill.show(ui)
		}
	}

	pub fn get(&self, ctx: &egui::Context, is_dark_mode: bool) -> (egui::TextureId, egui::Vec2) {
		if is_dark_mode {
			(self.white_fill.texture_id(ctx), self.white_fill.size_vec2())
		} else {
			(self.black_fill.texture_id(ctx), self.black_fill.size_vec2())
		}
	}
}