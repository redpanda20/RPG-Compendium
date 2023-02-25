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
	pub fn _empty() -> Self {
		let empty_svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="640" height="120">
		<style xmlns="" id="autoconsent-prehide"/></svg>"##;
		Self {
			white_fill: egui_extras::RetainedImage::from_svg_bytes("empty_white", empty_svg).unwrap(),
			black_fill: egui_extras::RetainedImage::from_svg_bytes("empty_black", empty_svg).unwrap(),
		}
	}

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

pub struct ImageBytes {
	pub account: Vec<u8>,
	pub account_none: Vec<u8>,
	pub toggle_off: Vec<u8>,
	pub toggle_on: Vec<u8>,
	pub no_profile_picture: Vec<u8>,
}

impl Default for ImageBytes {
    fn default() -> Self {
        Self { 
			account_none: 
			br##"<svg xmlns="http://www.w3.org/2000/svg" height="40" viewBox="0 96 960 960" width="40"><path d="M226 794q59-39.666 121-60.833T480 712q71 0 133.333 21.167Q675.667 754.334 734.667 794q41-49.667 59.833-103.667 18.834-54 18.834-114.333 0-141-96.167-237.167T480 242.666q-141 0-237.167 96.167T146.666 576q0 60.333 19.167 114.333T226 794Zm253.876-184.667q-58.209 0-98.043-39.957Q342 529.419 342 471.209q0-58.209 39.957-98.042 39.958-39.834 98.167-39.834t98.043 39.958Q618 413.248 618 471.457q0 58.21-39.957 98.043-39.958 39.833-98.167 39.833ZM479.73 976q-83.097 0-156.183-31.5t-127.15-85.833q-54.064-54.334-85.23-127.227Q80 658.546 80 575.667q0-82.88 31.5-155.773Q143 347 197.333 293q54.334-54 127.227-85.5Q397.454 176 480.333 176q82.88 0 155.773 31.5Q709 239 763 293t85.5 127Q880 493 880 575.823q0 82.822-31.5 155.666T763 858.667Q709 913 635.914 944.5T479.73 976Z"/></svg>"##.to_vec(),
			account:
			br##"<svg xmlns="http://www.w3.org/2000/svg" height="40" viewBox="0 96 960 960" width="40"><path d="M480 909.334q49 0 95.667-13.834 46.667-13.833 88.334-40.5L524.334 715.334q-11.333-1.333-22.167-2.334-10.833-1-22.167-1-68.333 0-132.667 21.834Q283 755.667 226 794q56.667 54.334 114.333 84.834Q398 909.334 480 909.334ZM805 995l-93.333-92.334q-50.333 36.001-109.334 54.667Q543.333 976 480 976q-83 0-156-31.5T197 859q-54-54-85.5-127T80 576q0-62.333 18.5-121T153 345L49.666 240.666q-10-10-9.5-23.833.5-13.833 10.5-23.833 10-10 23.834-10 13.833 0 23.833 10l754.334 755.333q10 10 10 23.334 0 13.333-10 23.333-10 10-23.833 10Q815 1005 805 995Zm1-187.666L572.666 574q21.667-19.667 33.501-46.167 11.833-26.5 11.833-56.5 0-57.333-40.333-97.666-40.334-40.334-97.667-40.334-30 0-56.5 11.834-26.5 11.833-46.167 33.5L248.666 250q51-36 110-55Q417.667 176 480 176q83 0 156 31.5T763 293q54 54 85.5 127T880 576q0 62.333-19 121.333-19 59.001-55 110.001Z"/></svg>"##.to_vec(),
			
			toggle_off: 
			br##"<svg xmlns="http://www.w3.org/2000/svg" height="48" viewBox="0 96 960 960" width="48" fill="#FFFFFF"><path d="M278 791q-89.583 0-152.292-62.707Q63 665.586 63 576.005q0-89.582 62.708-152.294Q188.417 361 278 361h404q89.583 0 152.292 62.707Q897 486.414 897 575.995q0 89.582-62.708 152.294Q771.583 791 682 791H278Zm.286-43.769H682q70.208 0 120.719-49.961 50.512-49.961 50.512-121.395 0-71.435-50.512-121.27Q752.208 404.769 682 404.769H278.286q-71.501 0-121.509 49.925-50.008 49.924-50.008 121.307 0 71.384 50.008 121.307t121.509 49.923ZM277.828 680q43.678 0 73.81-30.1 30.131-30.099 30.131-73.868 0-43.769-30.099-73.901Q321.57 472 277.891 472q-43.678 0-74.285 30.1Q173 532.199 173 575.968q0 43.769 30.575 73.901Q234.15 680 277.828 680ZM480 576Z"/></svg>"##.to_vec(),
			toggle_on:
			br##"<svg xmlns="http://www.w3.org/2000/svg" height="48" viewBox="0 96 960 960" width="48"><path d="M278 791q-89.583 0-152.292-62.707Q63 665.586 63 576.005q0-89.582 62.708-152.294Q188.417 361 278 361h404q89.583 0 152.292 62.707Q897 486.414 897 575.995q0 89.582-62.708 152.294Q771.583 791 682 791H278Zm404.109-111q43.678 0 74.285-30.1Q787 619.801 787 576.032q0-43.769-30.575-73.901Q725.85 472 682.172 472t-73.81 30.1q-30.131 30.099-30.131 73.868 0 43.769 30.099 73.901Q638.43 680 682.109 680Z"/></svg>"##.to_vec(),
			no_profile_picture:
			br##"<svg xmlns="http://www.w3.org/2000/svg" height="128" viewBox="0 96 960 960" width="128"><path d="M215.384 909q-27.782 0-48.083-20.301T147 840.616V311.384q0-27.782 20.301-48.083T215.384 243h529.232q27.782 0 48.083 20.301T813 311.384v529.232q0 27.782-20.301 48.083T744.616 909H215.384Zm0-43.769h529.232q9.23 0 16.923-7.692 7.692-7.693 7.692-16.923V311.384q0-9.23-7.692-16.923-7.693-7.692-16.923-7.692H215.384q-9.23 0-16.923 7.692-7.692 7.693-7.692 16.923v529.232q0 9.23 7.692 16.923 7.693 7.692 16.923 7.692Zm73.923-99.769h388.539L557.462 605.384 446.769 744.539l-72.461-88.77-85.001 109.693Zm-98.538 99.769V286.769v578.462Z"/></svg>"##.to_vec(),
		}
    }
}