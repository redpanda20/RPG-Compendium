use super::icon;
use super::defines;

use super::pages;
use super::popups;

use super::spells;
use super::character;

pub fn upper(parent: &mut super::App, ctx: &egui::Context, frame: &mut eframe::Frame) -> egui::InnerResponse<()> {
	#[allow(non_snake_case)]
	let HOME =
		icon::Icon::from_svg_responsive(
			defines::HOME.to_vec(), ctx);
	
	#[allow(non_snake_case)]
	let BOOKLET =
		icon::Icon::from_svg_responsive(
			defines::BOOKLET.to_vec(), ctx);
	
	#[allow(non_snake_case)]
	let DARK_MODE =
		icon::Icon::from_svg_responsive_precalculated(
			defines::TOGGLE_OFF.to_vec(),
			defines::TOGGLE_ON.to_vec(),
			ctx);

	#[allow(non_snake_case)]
	let VERT_MORE =
		icon::Icon::from_svg_responsive(
			defines::MORE_VERT.to_vec(), ctx);
		
	return egui::TopBottomPanel::top("Menu Bar - Upper").show(ctx, |ui| {
	egui::menu::bar(ui, |ui| {

		ui.spacing_mut().item_spacing = egui::vec2(12.0, 2.0);

		ui.add_space(12.0);

		let home = HOME.get(ctx);
		if ui.add_enabled(
			match parent.current_page {
				super::pages::Page::Home => false,
				_ => true,
			},
			egui::widgets::ImageButton::new(home.0, home.1)
		).clicked() {
			parent.current_page = super::pages::Page::Home
		};

		let booklet = BOOKLET.get(ctx);
		if ui.add_enabled(
			match parent.current_page {
				super::pages::Page::Compendium(_) => false,
				_ => true,
			},
			egui::widgets::ImageButton::new(booklet.0, booklet.1)
		).clicked() {
			parent.current_page = super::pages::Page::Compendium(spells::SpellType::None);
		};

		ui.add(
			egui::widgets::Separator::default()
				.shrink(2.0)
		);

		ui.add(
			egui::widgets::Label::new(
				egui::RichText::new("RPG Compendium")
				.heading()
				.strong())
		);
	
// Right hand side
		ui.with_layout(egui::Layout::right_to_left(egui::Align::Center) ,|ui| {	

			ui.add_space(12.0);

			let vert = VERT_MORE.get(ctx);
			
			ui.menu_image_button(vert.0, vert.1, |ui| {

				let dark_mode = DARK_MODE.get(ctx);
				ui.vertical_centered(|ui|

					if ui.add(
						egui::ImageButton::new(
							dark_mode.0,
							dark_mode.1,
						)
					).clicked() {
						if ctx.style().visuals.dark_mode {
							ctx.set_visuals(egui::Visuals::light())
						} else {
							ctx.set_visuals(egui::Visuals::dark())
					}}
				);
		
				ui.add(
					egui::widgets::Button::new(
						egui::RichText::new("Options"))
				);

				ui.add(
					egui::widgets::Button::new(
						egui::RichText::new("About"))
				);

				ui.add(
					egui::widgets::Button::new(
						egui::RichText::new("Credits"))
				);

			// Save button
				if ui.add(
					egui::Button::new("Save")
					.shortcut_text( egui::Context::format_shortcut(ctx, &super::shortcuts::SAVE))
				).clicked() {
					if let Some(storage) = frame.storage_mut() {
						eframe::App::save(parent, storage);
					};
					ui.close_menu();
				}
			

			// Close button: Innactive on web
				#[cfg(not(target_arch = "wasm32"))]
				if ui.add(
					egui::Button::new("Close")
					.shortcut_text( egui::Context::format_shortcut(ctx, &super::shortcuts::SHUTDOWN))
				).clicked() {
					frame.close()
				}
			});

			if ui.button("Character Sheet").clicked() {
				if parent.current_popup.is_none() {
					if parent.current_user.get_character().is_none() {
						parent.current_popup = popups::Popup::CreateCharacter(popups::CharacterDetails::new())
					} 
					parent.current_page = pages::Page::CharacterSheet(character::CharacterSheetDetails::new())
	
				}
			}

		});
	});
	});
}




pub fn lower(parent: &mut super::App, ctx: &egui::Context, _frame: &mut eframe::Frame) -> egui::InnerResponse<()> {
	#[allow(non_snake_case)]
	let ACCOUNT =
	icon::Icon::from_svg_responsive(
		defines::ACCOUNT.to_vec(), ctx);
	#[allow(non_snake_case)]
	let NO_ACCOUNT =
	icon::Icon::from_svg_responsive(
		defines::ACCOUNT.to_vec(), ctx);

	#[allow(non_snake_case)]
	let DARK_MODE =
	icon::Icon::from_svg_responsive_precalculated(
		defines::TOGGLE_OFF.to_vec(),
		defines::TOGGLE_ON.to_vec(),
		ctx);
		
	

	return egui::TopBottomPanel::bottom("Menu Bar - Lower").show(ctx, |ui| {

		egui::menu::bar(ui, |ui| {
			ui.spacing_mut().item_spacing = egui::vec2(12.0, 6.0);

			ui.add_space(12.0);

			if parent.current_user.is_logged_in() {
				let account = ACCOUNT.get(ctx);
				if ui.add(
					egui::widgets::Button::image_and_text(account.0, account.1, "Logged in")
				).clicked() {
					if parent.current_popup.is_none() {
						parent.current_popup = super::popups::Popup::ViewAccount
					}
				};

			} else {
				let no_account = NO_ACCOUNT.get(ctx);
				if ui.add(
					egui::widgets::Button::image_and_text(no_account.0, no_account.1, "Log in")
				).clicked() {
					if parent.current_popup.is_none() {
						parent.current_popup = super::popups::Popup::LogIn(
							super::popups::UserDetails::new(
								String::new(),
								String::new()))
					}
				};
	
			}
			
			ui.with_layout(
				egui::Layout::right_to_left(egui::Align::Center),
				|ui| {

				ui.add_space(12.0);

				ui.add(
					egui::widgets::Button::new(
						egui::RichText::new("Credits"))
				);

				ui.add(
					egui::widgets::Button::new(
						egui::RichText::new("About"))
				);

				ui.add(
					egui::widgets::Button::new(
						egui::RichText::new("Options"))
				);

				let dark_mode = DARK_MODE.get(ctx);

				if ui.add(
					egui::ImageButton::new(
						dark_mode.0,
						dark_mode.1,
					)
				).clicked() {
					if ctx.style().visuals.dark_mode {
						ctx.set_visuals(egui::Visuals::light())
					} else {
						ctx.set_visuals(egui::Visuals::dark())
				}}

			});
		});
	});
}