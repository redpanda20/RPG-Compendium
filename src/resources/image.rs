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

#[derive(Clone, PartialEq)]
enum ImageState {
	Unset,
	DefaultSet,
	ImageSet
}

#[derive(Clone)]
pub struct OptionalImage {
	image: Option<egui::TextureHandle>,
	size: [u32; 2],
	thumbnail: Option<egui::TextureHandle>,
	thumb_size: [u32; 2],
	state: ImageState,
}
impl Default for OptionalImage {
	
	fn default() -> Self {
		Self {
			image: None,
			size: [350, 350],
			thumbnail: None,
			thumb_size: [80, 80],
			state: ImageState::Unset
		}
	}
}
impl OptionalImage {
	
	pub fn update(&mut self, ctx: &egui::Context, raw_file: Vec<u8>) {
		let image = image_to_texture(raw_file.clone(), self.size);
		if let Some(handle) = &mut self.image {
			handle.set(image, Default::default());
		} else {
			self.image = Some(ctx.load_texture(
				"Image",
				image,
				Default::default()));	
		}

		let thumbnail = image_to_texture(raw_file.clone(), self.thumb_size);
		if let Some(handle) = &mut self.thumbnail {
			handle.set(thumbnail, Default::default());
		} else {
			self.image = Some(ctx.load_texture(
				"Thumbnail",
				thumbnail,
				Default::default()));	
		}
		self.state = ImageState::ImageSet;
	}

	pub fn is_set(&self) -> bool {
		return self.state == ImageState::ImageSet
	}

	fn set_default(&mut self, ctx: &egui::Context) -> (egui::TextureHandle, egui::TextureHandle) {
		let raw_image = match ctx.style().visuals.dark_mode {
			true => super::defines::SELECT_IMAGE_LIGHT.to_vec(),
			false => super::defines::SELECT_IMAGE.to_vec()
		};	
		let image = ctx.load_texture(
			"Optional Image Default",
			image_to_texture(raw_image.clone(), self.size),
			Default::default());
		let thumbnail = ctx.load_texture(
			"Optional Image Default",
			image_to_texture(raw_image, self.thumb_size),
			Default::default());
	
		self.image = Some(image.clone());
		self.thumbnail = Some(thumbnail.clone());
		self.state = ImageState::DefaultSet;

		return (image, thumbnail)
	}

	pub fn get(&mut self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		match self.state {
			ImageState::ImageSet => {
				if let Some(image) = &self.image {
					return (image.id(), image.size_vec2());
				}
				self.state = ImageState::Unset;
				return self.get(ctx);
			},
			ImageState::DefaultSet => {
				if let Some(image) = &self.image {
					return (image.id(), image.size_vec2());
				}
				self.state = ImageState::Unset;
				return self.get(ctx);
			},
			ImageState::Unset => {
				let (image, _) = self.set_default(ctx);
				return (image.id(), image.size_vec2())
			},
		};
	}

	pub fn get_thumbnail(&mut self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		match self.state {
			ImageState::ImageSet => {
				if let Some(image) = &self.thumbnail {
					return (image.id(), image.size_vec2());
				}
				self.state = ImageState::Unset;
				return self.get(ctx);
			},
			ImageState::DefaultSet => {
				if let Some(image) = &self.thumbnail {
					return (image.id(), image.size_vec2());
				}
				self.state = ImageState::Unset;
				return self.get(ctx);

			},
			ImageState::Unset => {
				let (_, image) = self.set_default(ctx);
				return (image.id(), image.size_vec2())
			},
		};
	}

}