use egui::*;

mod user;

mod images;
mod shortcuts;
mod menubar;
mod loader;

// Otherwise web compilation will complain about shutdown being unused

#[derive(serde::Serialize, serde::Deserialize)]
pub struct App {
	text: String,

	current_user: user::CurrentUser,

	#[serde(skip)]
	images: images::OptionalImage,

	#[serde(skip)]
	shortcuts: shortcuts::Shortcuts,

	#[serde(skip)]
	loader: loader::Loader,
}

impl Default for App {
    fn default() -> Self {
        Self {
			text: "Test".to_string(),
			current_user: user::CurrentUser::Empty(user::NewUser::default()),
			images: images::OptionalImage::default(),
            shortcuts: shortcuts::Shortcuts::default(),
			loader: loader::Loader::default(),
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
		if let Some(promise) = &self.loader.load_promise {

			if let Some(result) = promise.ready() {

				if let Ok((usage, raw_file)) = result {
					match usage {

						loader::FileUsage::ProfilePicture => {
							match &mut self.current_user {
								user::CurrentUser::LoggedIn(user) => {
									user.update_profile_picture(ctx, raw_file.to_vec())
								},
								_ => todo!(),
								}
							}
						_ => {
							self.images.load_image_from_raw(ctx, raw_file.to_vec())
							},
					}
				}

				self.loader.load_promise = None;
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