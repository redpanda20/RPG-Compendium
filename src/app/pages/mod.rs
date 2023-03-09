use super::icon;
use super::defines;

use super::spells;
use super::character;

use super::popups;

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Page {
	Home,
	#[serde(skip)]
	Compendium(spells::SpellType),
	CharacterSheet(character::CharacterSheetDetails)
}
pub fn show_home(parent: &mut super::App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	egui::CentralPanel::default().show(ctx, |ui| {
		ui.label("This is the main page");

		let booklet = icon::Icon::from_svg_constant(
			defines::BOOKLET.to_vec(), ctx)
			.get(ctx);
		if ui.add(
			egui::Button::image_and_text(booklet.0, booklet.1, "Compendium")	
		).clicked() {
			parent.current_page = Page::Compendium(spells::SpellType::None)
		};
	});
}
pub fn show_spells(parent: &mut super::App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	let Page::Compendium(selected_type) = parent.current_page.clone() else {
		return
	};
	let Some(content) = &parent.mystward_content else {
		return
	};


	egui::CentralPanel::default().show(ctx, |ui| {
	egui::ScrollArea::vertical().show(ui, |ui| {

		ui.horizontal(|ui|{
			let spacing = (ui.available_width() - 320.0) / 2.0;
			ui.add_space(spacing);
			ui.add_enabled_ui(
				match selected_type { spells::SpellType::None => false, _ => true },
				|ui| {
				if ui.add_sized(
					egui::vec2(80.0, 32.0),
					egui::Button::new("All")
				).clicked() {
					parent.current_page = Page::Compendium(spells::SpellType::None);
				}
			});
			ui.add_enabled_ui(
				match selected_type { spells::SpellType::Arcane(_) => false, _ => true },
				|ui| {
				if ui.add_sized(
					egui::vec2(80.0, 32.0),
					egui::Button::new("Arcane")
				).clicked() {
					parent.current_page = Page::Compendium(
						spells::SpellType::Arcane(std::collections::HashSet::new())
					);
				}
			});
			ui.add_enabled_ui(
				match selected_type { spells::SpellType::Fae(_) => false, _ => true },
				|ui| {
				if ui.add_sized(
					egui::vec2(80.0, 32.0),
					egui::Button::new("Fae")
				).clicked() {
					parent.current_page = Page::Compendium(
						spells::SpellType::Fae(spells::FaePatron::Generic)
					);
				}
			});
			ui.add_space(spacing);

		});

		ui.horizontal(|ui| {
			match selected_type {
				spells::SpellType::Arcane(_) => {
					let Page::Compendium(spells::SpellType::Arcane(active_concepts)) = &mut parent.current_page else { return }; 
					ui.add_space((ui.available_width() - 7.0 * (24.0 + 2.0 * ui.style().spacing.item_spacing.x + 2.0 * ui.style().spacing.button_padding.x)) / 2.0);
				
					let concepts = [
						(&spells::ArcaneConcept::Ignition, defines::IGNITION),
						(&spells::ArcaneConcept::Life, defines::LIFE),
						(&spells::ArcaneConcept::Design, defines::DESIGN),
						(&spells::ArcaneConcept::Astral, defines::ASTRAL),
						(&spells::ArcaneConcept::Force, defines::FORCE),
						(&spells::ArcaneConcept::Widsom, defines::WISDOM),
						(&spells::ArcaneConcept::Entropy, defines::ENTROPY),
					];

					for (concept, img) in concepts {
						let (texture_id, size) = icon::Icon::from_svg_constant(img.to_vec(), ctx).get(ctx);
						ui.vertical(|ui| {
							if ui.add(
								egui::ImageButton::new(texture_id, size)
							).clicked() {
								if !active_concepts.insert(concept.clone()) {
									active_concepts.remove(concept);
								}
							};
							if active_concepts.contains(concept) {
								let pos = ui.next_widget_position();
								let painter = ui.painter();
								let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
								painter.hline(pos.x..=(pos.x + 24.0 + 2.0 * ui.style().spacing.button_padding.x), pos.y, stroke);
							}
						});	
					}

				},
				spells::SpellType::Fae(_) => {
					let Page::Compendium(spells::SpellType::Fae(active_patron)) = &mut parent.current_page else { return };
					ui.add_space((ui.available_width() - 4.0 * (24.0 + 2.0 * ui.style().spacing.item_spacing.x + 2.0 * ui.style().spacing.button_padding.x)) / 2.0);
					
					let patrons = [
						(spells::FaePatron::Pixie, defines::PIXIE),
						(spells::FaePatron::Sylviel, defines::SYLVIEL),
						(spells::FaePatron::ForgeSprite, defines::FORGE_SPRITE),
					];
					for (patron, img) in patrons {
						let (texture_id, size) = icon::Icon::from_svg_constant(img.to_vec(), ctx).get(ctx);
						ui.vertical(|ui| {
							if ui.add(
								egui::ImageButton::new(texture_id, size)
							).clicked() {
								if *active_patron == patron {
									*active_patron = spells::FaePatron::Generic;
								} else {
									*active_patron = patron
								}
							};
							if *active_patron == patron {
								let pos = ui.next_widget_position();
								let painter = ui.painter();
								let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
								painter.hline(pos.x..=(pos.x + 24.0 + 2.0 * ui.style().spacing.button_padding.x), pos.y, stroke);
							}
						});
					}
				},
				spells::SpellType::None => (),
				spells::SpellType::Wild => (),
				
			}
		});

		let filtered_spells: Vec<spells::Spell> = match selected_type {
			spells::SpellType::None => {
				content.all_spells.clone()
			},
			spells::SpellType::Wild => {
				content.all_spells.clone()
			},
			spells::SpellType::Arcane(selected_concepts) => {
				content.all_spells.clone()
					.into_iter()
					.filter(|spell| match &spell.spell_type {
						spells::SpellType::Arcane(concepts) => selected_concepts.is_empty() || !concepts.is_disjoint(&selected_concepts),
						_ => false})
					.collect()
			},
			spells::SpellType::Fae(selected_patron) => {
				content.all_spells.clone()
					.into_iter()
					.filter(|spell| match spell.spell_type {
					spells::SpellType::Fae(patron) => selected_patron == spells::FaePatron::Generic || patron == selected_patron,
					_ => false })
					.collect()
			},
		};

		let content_width = (ui.available_width() / 6.0).clamp(200.0, 340.0);
		let row_size = (((ui.available_width() - 200.0) / content_width).floor() as usize).clamp(1, 5);
		let spacing = (ui.available_width() - content_width * (row_size) as f32 - 10.0 * (row_size - 1) as f32) / 2.0;	

		ui.add_space(5.0);
		ui.add( egui::Separator::default().horizontal().shrink( spacing ));
		ui.add_space(10.0);

		ui.horizontal_top(|ui| {

			ui.add_space(spacing);

			egui::Grid::new("content_grid")
				.max_col_width(content_width)
				.spacing(egui::vec2(10.0, 10.0))
				.show(ui, |ui| {
				let mut row_length = 0;
				for spell in filtered_spells {
					spell.show(ui, ctx);

					row_length += 1;
					if row_length >= row_size {
						row_length = 0;
						ui.end_row();
					}
				}
			});

			ui.add_space(spacing);

		});

	});
	});
}

pub fn show_character(parent: &mut super::App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	let Page::CharacterSheet(details) = &mut parent.current_page else {
		return
	};	

	egui::CentralPanel::default().show(ctx, |ui| {
		egui::ScrollArea::vertical()
			.auto_shrink([false, false])
			.drag_to_scroll(true)
			.show(ui, |ui| {


			if let Some(character) = parent.current_user.get_character() {

			let width = (ui.available_width() - 100.0).clamp(200.0, 1500.0);
			let spacing = (ui.available_width() - width) / 2.0;

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
					parent.current_user.remove_character();
				};
			});

			ui.add_space(40.0);

			} else {

				ui.label("No character available");
				if ui.button("Create character").clicked() {
					if parent.current_popup.is_none() {
						parent.current_popup = popups::Popup::CreateCharacter(popups::CharacterDetails::new());
					}

				};
			};
		});

	});
}