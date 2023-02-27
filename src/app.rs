use egui::*;

mod user;
mod shortcuts;
mod images;
mod menubar;
mod popups;
mod loader;

#[derive(PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
enum Page {
	Home,
	Compendium,
	Character
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct App {
	text: String,

	past_users: std::collections::HashMap<String, user::User>,
	current_user: user::User,

	current_page: Page,

	#[serde(skip)]
	current_popup: popups::Popup,

	#[serde(skip)]
	loader: loader::Loader,
}

impl Default for App {
    fn default() -> Self {
        Self {
			text: "Test".to_string(),
			past_users: std::collections::HashMap::new(),
			current_user: user::User::default(),

			loader: loader::Loader::default(),
            current_page: Page::Home,
			current_popup: popups::Popup::None,
		}
    }
}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

		let Some(storage) = cc.storage else {
			return Self::default()
		};
		eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
	}
}

impl eframe::App for App {

	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self)
	}

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
		// Check ongoing promise and handle result
		for option in &mut self.loader.promises {
			let Some(promise) = option else {
				break;
			};
			let Some((file_usage, file_raw)) = promise.ready() else {
				break;
			};
			match file_usage {
				loader::FileUsage::ProfilePicture => {
							if self.current_user.is_mutable() && self.current_user.is_logged_in() {
								self.current_user.update_profile_picture(ctx, file_raw.to_vec());
							}
						},

						loader::FileUsage::Error => (),
			};
		}
		// Handle shortcut inputs
		ctx.input_mut(|i| {
			// Cannot shutdown web application
			#[cfg(not(target_arch = "wasm32"))]
			if i.consume_shortcut(&shortcuts::SHUTDOWN) {
				frame.close()
			}
			if i.consume_shortcut(&shortcuts::SAVE) {
				if let Some(storage) = frame.storage_mut() {
					self.save(storage);
				}
			}
		});

		match self.current_page {
			Page::Home => {
				menubar::lower(self, ctx, frame);

				CentralPanel::default().show(ctx, |ui| {
					ui.label("This is the main page");

					let booklet = images::StaticSvg::new(
						String::from("Booklet"),
						images::BOOKLET.to_vec())
						.get(ctx);
					if ui.add(
						egui::Button::image_and_text(booklet.0, booklet.1, "Compendium")	
					).clicked() {
						self.current_page = Page::Compendium
					};
				});
			},
			Page::Compendium => {
				menubar::upper(self, ctx, frame);

				TopBottomPanel::bottom("References").show(ctx, |ui| {
					ui.label("Built by Alexandra Stephens");
					ui.hyperlink_to("Built using eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
				});

				CentralPanel::default().show(ctx, |ui| {
					ui.label("There should be something here!");
					ui.text_edit_multiline(&mut self.text);
				});
			}
			Page::Character => {
				menubar::upper(self, ctx, frame);

				CentralPanel::default().show(ctx, |ui| {
					ui.label("Character here");
				});
			},
		}
		if self.current_popup != popups::Popup::None {
			ctx.layer_painter(
				egui::LayerId {
					order: egui::layers::Order::Background,
					id: egui::Id::new("paint_layer")})
				.rect_filled(
					egui::Rect::EVERYTHING,
					egui::Rounding{ nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
					egui::Color32::from_rgba_unmultiplied(0, 0, 0, 96))
		}
		match self.current_popup {
			popups::Popup::LogIn(_) => {
				popups::show_login(self, ctx);
			},
			popups::Popup::CreateAccount(_) => {
				popups::show_signup(self, ctx);
			},
			popups::Popup::ViewAccount => {
				popups::show_account(self, ctx);
			},
			popups::Popup::None => (),
		};

    }
}