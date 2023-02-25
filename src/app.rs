use egui::*;
use poll_promise::Promise;

mod images;
mod shortcuts;
mod menubar;

// Otherwise web compilation will complain about shutdown being unused

#[derive(serde::Serialize, serde::Deserialize)]
pub struct App {
    text: String,

	#[serde(skip)]
	images: images::Images,

	#[serde(skip)]
	shortcuts: shortcuts::Shortcuts,

	#[serde(skip)]
	load_promise: Option<Promise<Result<Vec<u8>, &'static str>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
			text: "Test".to_string(),
			images: images::Images::default(),
            shortcuts: shortcuts::Shortcuts::default(),
			load_promise: None,
		}
    }
}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
		}

		Self::default()
	}
}

impl eframe::App for App {
	
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self)
	}

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
		// Check ongoing promise and handle result
		if let Some(promise) = &self.load_promise {

			if let Some(result) = promise.ready() {

				if let Ok(raw_file) = result {
					self.images.load_image_from_raw(ctx, raw_file.to_vec());
				}

				self.load_promise = None;
			}
		}

		// Handle shortcut inputs
		ctx.input_mut(|i| {
			// Cannot shutdown web application
			#[cfg(not(target_arch = "wasm32"))]
			if i.consume_shortcut(&self.shortcuts.shutdown) {
				frame.close()
			}
			if i.consume_shortcut(&self.shortcuts.save) {
				if let Some(storage) = frame.storage_mut() {
					self.save(storage);
				}
			}
		});

		menubar::show_menu_bar(self, ctx, frame);

		TopBottomPanel::bottom("References").show(ctx, |ui| {
			ui.label("Built by Alexandra Stephens");
			ui.hyperlink_to("Built using eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
		});

		CentralPanel::default().show(ctx, |ui| {
			ui.text_edit_multiline(&mut self.text);
		});
    }
}