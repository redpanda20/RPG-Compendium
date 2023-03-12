
use super::*;

fn character_preview(ctx: &egui::Context, ui: &mut egui::Ui, width: f32, character: &mut character::Character) -> egui::Response {
	let mut option_response: Option<egui::Response> = None;
	ui.vertical(|ui| {
		ui.add_space(20.0);
		let (rect, response) = ui.allocate_at_least(
			egui::vec2(width, 100.0),
			egui::Sense::click());
		let visuals = ui.style().interact(&response);
		ui.painter().rect(
			rect,
			visuals.rounding,
			visuals.weak_bg_fill,
			visuals.bg_stroke);
	
		ui.allocate_ui_at_rect(rect.shrink2(egui::vec2(10.0, 5.0)), |ui| {
			ui.horizontal(|ui|{
				ui.set_min_height(rect.shrink2(egui::vec2(10.0, 5.0)).height());
				ui.horizontal_centered(|ui| {
					character.get_picture_static(ctx, ui);
				});
				ui.vertical(|ui| {
					ui.label(egui::RichText::new(character.character_info.name.clone()).size(24.0));
					ui.label(character.archetype.to_string());				
				});
			});
		});
		option_response = Some(response);
	});
	return option_response.unwrap();
}

pub fn show_all_characters(parent: &mut App, ctx: &egui::Context) {
	egui::CentralPanel::default().show(ctx, |ui| {
		egui::ScrollArea::vertical().show(ui, |ui| {

		let width = (ui.available_width() - 100.0).clamp(200.0, 1500.0);
		let spacing = (ui.available_width() - width) / 2.0;

		ui.vertical(|ui| {

				let mut num_characters: usize = 0;
				let mut active_name: Option<String> = None;
				let mut new_active: Option<character::Character> = None;

				// Active character
				if let Some(character) = parent.current_user.get_active_character() {
					active_name = Some(character.character_info.name.clone());
					num_characters += 1;

					ui.add_space(20.0);
					ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Active").size(24.0));
					});				

					ui.horizontal(|ui| {
						ui.add_space(spacing);
						if character_preview(ctx, ui, width, character).clicked() {
							parent.current_page = Page::CharacterSheet(Default::default());
						}	
					});

					ui.add_space(10.0);
					ui.add(egui::Separator::default().shrink(spacing));
				}

				// Inactive characters
				for character in parent.current_user.get_all_characters() {
					if let Some(name) = &active_name { // Ignore active character (should exist)
						if &character.character_info.name == name {
							continue;
						}
					}
					num_characters += 1;

					ui.horizontal(|ui| {
						ui.add_space(spacing);
						if character_preview(ctx, ui, width, character).clicked() {
							new_active = Some(character.clone());
						}	
					});
				}

				if let Some(character) = new_active {
					parent.current_page = Page::CharacterSheet(Default::default());
					parent.current_user.set_active_character(character);
				}
				
				ui.add_space(20.0);

				ui.vertical_centered(|ui| {
					match num_characters {
						0 => {
							ui.label("No characters found. Would you like to make one?");
							if ui.button("Create character").clicked() {
								if parent.current_popup.is_none() {
									parent.current_popup = popups::Popup::CreateCharacter(Default::default());
								}
							}		
						},
						1 | 2 => {
							ui.label("Would you like to make a new character?");
							if ui.button("Create character").clicked() {
								if parent.current_popup.is_none() {
									parent.current_popup = popups::Popup::CreateCharacter(Default::default());
								}
							}		
						}
						_ => {
							ui.label("Maximum number of characters reached");
							ui.label("To make a new character, please delete an existing character first");
						}
					}
				});

				// ui.add_space(ui.available_height().max(20.0));
		});
		});
	});
}

pub fn show_character(parent: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	egui::CentralPanel::default().show(ctx, |ui| {
		egui::ScrollArea::vertical()
			.auto_shrink([false, false])
			.drag_to_scroll(true)
			.show(ui, |ui| {

			let width = (ui.available_width() - 100.0).clamp(200.0, 1500.0);
			let spacing = (ui.available_width() - width) / 2.0;

			// Must be first due to parent.current page being borrowed as mut
			// Must be inside closure so that parent.current_user is in scope
			let Some(character) = parent.current_user.get_active_character() else {
				parent.current_page = Page::AllCharacters;
				return
			};
			let Page::CharacterSheet(details) = &mut parent.current_page else {
				return
			};	
				
			ui.horizontal(|ui| {
				ui.add_space(spacing);
				character.show(&mut parent.loader, details, ui, ctx, width);
			});

			ui.add_space(20.0);

			ui.vertical_centered(|ui| {
				if ui.add(
					egui::Button::new("Delete Character")
						.frame(true)
						.fill(egui::Color32::DARK_RED)
				).on_hover_text("There is no confirmation screen")
				.clicked() {
					parent.current_user.remove_active_character();
				};
			});

			ui.add_space(40.0);

		});

	});
}