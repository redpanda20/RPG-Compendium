use egui::TextureId;

fn load_bitmap(bytes: Vec<u8>) -> egui::ColorImage {
	let result = image::io::Reader::new(std::io::Cursor::new(bytes))
	.with_guessed_format()
	.expect("Error loading iamge")
	.decode();

	// Handle errors
	let image = if let Ok(image) = result {
		let size = [image.width() as _, image.height() as _];
		let image_buffer = image.to_rgba8();
		let pixels = image_buffer.as_flat_samples();
		egui::ColorImage::from_rgb(size, pixels.as_slice())
	} else {
		egui::ColorImage::new([24, 24], egui::Color32::RED)
	};
	image
}
pub fn from_png_constant(img_bytes: Vec<u8>, ctx: &egui::Context) -> Icon {

	let texture = ctx.load_texture("icon", load_bitmap(img_bytes), Default::default());

	Icon {
		image: texture.id(),
		alt_image: None }
}
pub fn from_png_responsive(light_mode_bytes: Vec<u8>, dark_mode_bytes: Vec<u8>, ctx: &egui::Context) -> Icon {
	
	let light_mode = ctx.load_texture("light_icon", load_bitmap(light_mode_bytes), Default::default());
	let dark_mode = ctx.load_texture("dark_icon", load_bitmap(dark_mode_bytes), Default::default());

	Icon {
		image: light_mode.id(),
		alt_image: Some(dark_mode.id()) }
}


pub struct Icon {

	image: TextureId,
	alt_image: Option<TextureId>,
}
impl Icon {

	pub fn from_svg_constant(img_bytes: Vec<u8>, ctx: &egui::Context) -> Self {
		let temp_image = egui_extras::RetainedImage::from_svg_bytes("svg icon", &img_bytes).unwrap();
		return Self {
			image: temp_image.texture_id(ctx),
			alt_image: None}
	}
	
	pub fn from_svg_responsive(img_bytes: Vec<u8>, ctx: &egui::Context) -> Self {	
		let icon_size = egui_extras::image::FitTo::Size(24, 24);

		// Get position of first '>'
		let (index, _) = img_bytes.clone().iter().enumerate().find(|(_, b)| **b == b'>').unwrap();
		// Split into left and right at position
		let (left, right) = img_bytes.split_at(index);
		let white_bytes = [left, br##" fill="#FFFFFF""##, right].concat();
		
		let alt_image = egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &white_bytes, icon_size).unwrap();

		// Calculate white image
		let image = egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &img_bytes.clone(), icon_size).unwrap();
	
		Self {
			image: image.texture_id(ctx),
			alt_image: Some(alt_image.texture_id(ctx))}
	}
		
	pub fn from_svg_responsive_precalculated(image_bytes: Vec<u8>, alt_bytes: Vec<u8>, ctx: &egui::Context) -> Self {	
		let icon_size = egui_extras::image::FitTo::Size(24, 24);

		let image = egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &alt_bytes.clone(), icon_size).unwrap();
		let alt_image = egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &image_bytes.clone(), icon_size).unwrap();

		Self {
			image: image.texture_id(ctx),
			alt_image: Some(alt_image.texture_id(ctx))}
	}

	pub fn get(&self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		let Some(alt) = self.alt_image.as_ref() else {
			return (self.image, egui::vec2(24.0, 24.0))
		};
		if ctx.style().visuals.dark_mode {
			(*alt, egui::vec2(24.0, 24.0))
		} else {
			(self.image, egui::vec2(24.0, 24.0))
		}
	}
}