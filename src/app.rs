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
	images: images::ImageBytes,

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

			images: images::ImageBytes::default(),
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
		for option in &mut self.loader.promises {

			if let Some(promise) = option {
				if let Some((file_usage, file_raw)) = promise.ready() {

					match file_usage {

						loader::FileUsage::ProfilePicture => {
							if let user::CurrentUser::LoggedIn(user) = &mut self.current_user {
								user.update_profile_picture(ctx, file_raw.to_vec());
							}
						},

						loader::FileUsage::Error => (),

					}
				}
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