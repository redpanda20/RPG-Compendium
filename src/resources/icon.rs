use egui::TextureId;

pub struct Icon {

	image: TextureId,
	alt_image: Option<TextureId>,
	size: egui::Vec2,
}
impl Icon {

	pub fn from_svg_constant(img_bytes: Vec<u8>, ctx: &egui::Context) -> Self {
		let temp_image = egui_extras::RetainedImage::from_svg_bytes("svg icon", &img_bytes).unwrap();
		return Self {
			image: temp_image.texture_id(ctx),
			alt_image: None,
			size: temp_image.size_vec2() }
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
		
		let alt_image = egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &white_bytes, icon_size).unwrap();

		// Calculate white image
		let image = egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &img_bytes.clone(), icon_size).unwrap();
	
		return Self {
			image: image.texture_id(ctx),
			alt_image: Some(alt_image.texture_id(ctx)),
			size: image.size_vec2()
		};
	}
	
	pub fn from_svg_responsive_precalculated(image_bytes: Vec<u8>, alt_bytes: Vec<u8>, ctx: &egui::Context) -> Self {	
		let icon_size = egui_extras::image::FitTo::Size(24, 24);

		let image = egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &alt_bytes.clone(), icon_size).unwrap();
		let alt_image = egui_extras::RetainedImage::from_svg_bytes_with_size("svg icon", &image_bytes.clone(), icon_size).unwrap();

		return Self {
			image: image.texture_id(ctx),
			alt_image: Some(alt_image.texture_id(ctx)),
			size: image.size_vec2()
		};
	}

	pub fn get(&self, ctx: &egui::Context) -> (egui::TextureId, egui::Vec2) {
		let Some(alt) = self.alt_image.as_ref() else {
			return (self.image, self.size)
		};
		if ctx.style().visuals.dark_mode {
			(*alt, self.size)
		} else {
			(self.image, self.size)
		}
	}
}