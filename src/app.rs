use egui::*;

use crate::resources::{loader, icon, defines};
use crate::mystward::{self, spells, character};

use crate::shortcuts;
use crate::user;

mod pages;
pub mod popups;
mod menubar;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct App {
	text: String,

	past_users: std::collections::HashMap<String, user::User>,
	pub current_user: user::User,

	pub current_page: pages::Page,

	#[serde(skip)]
	current_popup: popups::Popup,

	#[serde(skip)]
	pub loader: loader::Loader,

	#[serde(skip)]
	pub mystward_content: Option<mystward::Content>,
}

impl Default for App {
    fn default() -> Self {
        Self {
			text: "Test".to_string(),
			past_users: std::collections::HashMap::new(),
			current_user: user::User::default(),

			loader: loader::Loader::default(),
			// TODO! remove once more features added
            current_page: pages::Page::Compendium(spells::SpellType::None),
			current_popup: popups::Popup::None,

			mystward_content: None,
		}
    }
}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

		let Some(storage) = cc.storage else {
			return Self::default()
		};
		let mut new: App = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		new.mystward_content = Some(mystward::new(&cc.egui_ctx));

		return new
	}
}

impl eframe::App for App {

	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self)
	}

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
		let mut should_save = false;

		// Check reciever
		self.loader.update(ctx, &mut self.current_user);

		// Handle shortcut inputs
		ctx.input_mut(|i| {
			i.raw.pixels_per_point = frame.info().native_pixels_per_point;

			// Cannot shutdown web application
			#[cfg(not(target_arch = "wasm32"))]
			if i.consume_shortcut(&shortcuts::SHUTDOWN) {
				frame.close()
			}
			if i.consume_shortcut(&shortcuts::SAVE) {
				should_save = true;
			}
		});

		match self.current_page {
			pages::Page::Home => {
				menubar::lower(self, ctx, frame);
				pages::show_home(self, ctx, frame);
			},
			pages::Page::Compendium(_) => {
				menubar::upper(self, ctx, frame);
				pages::show_spells(self, ctx, frame);
			},
			pages::Page::AllCharacters => {
				menubar::upper(self, ctx, frame);
				pages::show_all_characters(self, ctx);
			}
			pages::Page::CharacterSheet(_) => {
				menubar::upper(self, ctx, frame);
				pages::show_character(self, ctx, frame);
			},
		}

		if !self.current_popup.is_none() {
			ctx.layer_painter(
				egui::LayerId {
					order: egui::layers::Order::Background,
					id: egui::Id::new("paint_layer")})
				.rect_filled(
					egui::Rect::EVERYTHING,
					egui::Rounding { nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
					egui::Color32::from_rgba_unmultiplied(0, 0, 0, 96))
		}
		match self.current_popup {
			popups::Popup::LogIn(_) => {
				popups::show_login(self, ctx);
			},
			popups::Popup::CreateAccount(_) => {
				let (save, _) = popups::show_signup(self, ctx);
				if save { should_save = true; }
			},
			popups::Popup::ViewAccount => {
				popups::show_account(self, ctx);
			},
			popups::Popup::CreateCharacter(_) => {
				popups::show_create_character(self, ctx);
			},
			popups::Popup::AdvanceCharacter(_) => {
				popups::show_advance_character(self, ctx);
			},
			popups::Popup::None => (),
		};

		if should_save {
			if let Some(storage) = frame.storage_mut() {
				self.save(storage);
			}
		}
    }
}