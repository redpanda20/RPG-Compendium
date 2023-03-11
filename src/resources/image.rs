// Stores single copy of picture that can be referenced elsewhere

fn image_to_texture(image: Vec<u8>, max_size: [u32; 2]) -> egui::ColorImage {
	let result = image::io::Reader::new(std::io::Cursor::new(image))
		.with_guessed_format()
		.expect("Error loading iamge")
		.decode();

	let Ok(mut image) = result else {
		return egui::ColorImage::new([30, 30], egui::Color32::RED);
	};
	image = image.resize(
		max_size[0],
		max_size[1],
		image::imageops::Lanczos3);

	let size = [image.width() as _, image.height() as _];
	let image_buffer = image.to_rgba8();
	let pixels = image_buffer.as_flat_samples();

	return egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
}

#[derive(Clone)]
pub struct OptionalImage {
	image: Option<egui::TextureHandle>,
	size: [u32; 2],
	default: Option<egui::TextureHandle>,
	is_set: bool,
}
impl Default for OptionalImage {
	
	fn default() -> Self {
		Self {
			image: None,
			size: [350, 350],
			default: None,
			is_set: false
		}
	}
}
impl OptionalImage {
	
	fn update(&mut self, ctx: &egui::Context, image: impl Into<egui::ImageData>) {
		let Some(handle) = self.image.as_mut() else {
			self.image = Some(ctx.load_texture(
				"Optional Image",
				image,
				Default::default()
			));
			self.is_set = true;
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
			image_to_texture(raw_file, self.size)
		);
	}

	pub fn is_set(&self) -> bool {
		return self.is_set
	}

	pub fn get(&mut self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		if self.is_set {
			if let Some(image) = &self.image {
				return (image.id(), image.size_vec2())
			} else {
				self.is_set = false;
			}
		}

		if self.default.is_none() {
			let raw_image = match ctx.style().visuals.dark_mode {
				true => super::defines::SELECT_IMAGE_LIGHT.to_vec(),
				false => super::defines::SELECT_IMAGE.to_vec()
			};
			self.default = Some(ctx.load_texture(
				"Optional Image Default",
				image_to_texture(raw_image, self.size),
				Default::default()));
		}
		let image = self.default.as_ref().unwrap();
		
		return (image.id(), image.size_vec2())
	}
}