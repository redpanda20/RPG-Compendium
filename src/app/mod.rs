use egui::*;

use crate::resources::{loader, images};
use crate::mystward::{spells};

use crate::shortcuts;
use crate::user;

mod popups;
mod menubar;
mod pages;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct App {
	text: String,

	past_users: std::collections::HashMap<String, user::User>,
	current_user: user::User,

	current_page: pages::Page,

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
            current_page: pages::Page::Home,
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
			pages::Page::Home => {
				menubar::lower(self, ctx, frame);
				pages::show_home(self, ctx, frame);
			},
			pages::Page::Compendium(_) => {
				menubar::upper(self, ctx, frame);
				pages::show_spells(self, ctx, frame);
			},
			pages::Page::Character => {
				menubar::upper(self, ctx, frame);
				pages::show_character(self, ctx, frame);
			},
		}

		if self.current_popup != popups::Popup::None {
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
				popups::show_signup(self, ctx);
			},
			popups::Popup::ViewAccount => {
				popups::show_account(self, ctx);
			},
			popups::Popup::None => (),
		};

    }
}