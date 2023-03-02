// use egui::TextureId;

pub struct Icon {

	// image: TextureId,
	// size: (f32, f32),

	image: egui_extras::RetainedImage,
	alt_image: Option<egui_extras::RetainedImage>, 
}
impl Icon {

	pub fn from_svg_constant(img_bytes: Vec<u8>, ctx: &egui::Context) -> Self {
		return Self {
			image: egui_extras::RetainedImage::from_svg_bytes("svg icon", &img_bytes).unwrap(),
			alt_image: None }
	}
	
	pub fn from_svg_responsive(img_bytes: Vec<u8>, ctx: &egui::Context) -> Self {
		const ICON_SIZE: [u32; 2] = [24, 24];
	
		return Self::from_svg_responsive_with_size(img_bytes, ICON_SIZE, ctx)
	}
	
	pub fn from_svg_responsive_with_size(img_bytes: Vec<u8>, size: [u32; 2], ctx: &egui::Context) -> Self {
		let icon_size = egui_extras::image::FitTo::Size(size[0], size[1]);
		// Get position of first '>'
		let (index, _) = img_bytes.clone().iter().enumerate().find(|(_, b)| **b == b'>').unwrap();
		// Split into left and right at position
		let (left, right) = img_bytes.split_at(index);
		let white_bytes = [left, br##" fill="#FFFFFF""##, right].concat();
		
	
		// Calculate white image
		let black_bytes = img_bytes.clone();
	
		return Self {
			image: egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &white_bytes, icon_size).unwrap(),
			alt_image: Some(egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &black_bytes, icon_size).unwrap()),
			};
	}
	
	pub fn from_svg_responsive_precalculated(white_bytes: Vec<u8>, black_bytes: Vec<u8>, ctx: &egui::Context) -> Self {	
		return Self {
			image: egui_extras::RetainedImage::from_svg_bytes("svg icon", &white_bytes).unwrap(),
			alt_image: Some(egui_extras::RetainedImage::from_svg_bytes("svg icon", &black_bytes).unwrap()),
			};
	}

	pub fn get(&self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		let Some(alt) = self.alt_image.as_ref() else {
			return (self.image.texture_id(ctx), self.image.size_vec2())
		};
		if ctx.style().visuals.dark_mode {
			(self.image.texture_id(ctx), self.image.size_vec2())
		} else {
			(alt.texture_id(ctx), alt.size_vec2())
		}
	}
}