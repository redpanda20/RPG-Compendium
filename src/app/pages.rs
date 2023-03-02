use super::images as images;
use super::menubar as menubar;
use super::spells as spells;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Page {
	Home,
	#[serde(skip)]
	Compendium(spells::SpellType),
	Character
}
pub fn show(parent: &mut super::App, ctx: &egui::Context, frame: &mut eframe::Frame) {
	match &parent.current_page {
		Page::Home => {
			menubar::lower(parent, ctx, frame);

			egui::CentralPanel::default().show(ctx, |ui| {
				ui.label("This is the main page");

				let booklet = images::StaticSvg::new(
					String::from("Booklet"),
					images::BOOKLET.to_vec())
					.get(ctx);
				if ui.add(
					egui::Button::image_and_text(booklet.0, booklet.1, "Compendium")	
				).clicked() {
					parent.current_page = Page::Compendium(spells::SpellType::None)
				};
			});
		},
		Page::Compendium(selected) => {
			let selected_type = selected.to_owned();
			menubar::upper(parent, ctx, frame);

			let all_spells = spells::get_all_spells(ctx);

			egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().show(ui, |ui| {

				ui.horizontal(|ui|{
					ui.add_space((ui.available_width() - 3.0 * 80.0 - 2.0 * ui.spacing().item_spacing.x) / 2.0);
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
				});

				ui.add_space(4.0);

				ui.horizontal(|ui| {
					match selected_type {
						spells::SpellType::Arcane(_) => {
							let Page::Compendium(spells::SpellType::Arcane(active_concepts)) = &mut parent.current_page else { return }; 
							ui.add_space((ui.available_width() - 7.0 * (24.0 + 2.0 * ui.spacing().button_padding.x) - 6.0 * ui.spacing().item_spacing.x) / 2.0);
						
							let concepts = [
								("Ignition", &spells::ArcaneConcept::Ignition, images::IGNITION),
								("Life", &spells::ArcaneConcept::Life, images::LIFE),
								("Design", &spells::ArcaneConcept::Design, images::DESIGN),
								("Astral", &spells::ArcaneConcept::Astral, images::ASTRAL),
								("Force", &spells::ArcaneConcept::Force, images::FORCE),
								("Wisdom", &spells::ArcaneConcept::Widsom, images::WISDOM),
								("Entropy", &spells::ArcaneConcept::Entropy, images::ENTROPY),
							];

							for (name, concept, img) in concepts {
								let (texture_id, size) = images::StaticSvg::new_single(String::from("concept"), img.to_vec()).get(ctx);
								ui.vertical(|ui| {
									if ui.add(
										egui::ImageButton::new(texture_id, size)
									)
									.on_hover_text(name)
									.clicked() {
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
							ui.add_space((ui.available_width() - 3.0 * (24.0 + 2.0 * ui.spacing().button_padding.x) - 2.0 * ui.spacing().item_spacing.x) / 2.0);
							
							let patrons = [
								("Pixie", spells::FaePatron::Pixie, images::PIXIE),
								("Sylviel", spells::FaePatron::Sylviel, images::SYLVIEL),
								("Forge Sprite", spells::FaePatron::ForgeSprite, images::FORGE_SPRITE),
							];
							for (name, patron, img) in patrons {
								let (texture_id, size) = images::StaticSvg::new_single(String::from("patron"), img.to_vec()).get(ctx);
								ui.vertical(|ui| {
									if ui.add(
										egui::ImageButton::new(texture_id, size)
									)
									.on_hover_text(name)
									.clicked() {
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
						
					}
				});

				// Calculate full match
				let spells_full: Vec<spells::Spell> = all_spells.clone()
					.into_iter()
					.filter(|spell| match selected_type.clone() {
						spells::SpellType::None => true,

						spells::SpellType::Arcane(concepts) => match &spell.spell_type {
								spells::SpellType::Arcane(spell_concepts) => spell_concepts.is_subset(&concepts),
								_ => false
							},

						spells::SpellType::Fae(patron) => match spell.spell_type {
								spells::SpellType::Fae(spell_patron) => spell_patron == patron,
								_ => false
							},
					})
					.collect();


				// Calculate partial match
				let spells_partial: Vec<spells::Spell> = all_spells.clone()
					.into_iter()
					.filter(|spell| match selected_type.clone() {
						spells::SpellType::None => false,

						spells::SpellType::Arcane(concepts) => match &spell.spell_type {
								spells::SpellType::Arcane(spell_concepts) => concepts.is_empty() || !spell_concepts.is_disjoint(&concepts),
								_ => false
							},

						spells::SpellType::Fae(patron) => match spell.spell_type {
								spells::SpellType::Fae(spell_patron) => patron == spells::FaePatron::Generic || spell_patron == spells::FaePatron::Generic,
								_ => false
							},
					})
					.collect();


				let content_width = (ui.available_width() / 6.0).clamp(200.0, 340.0);
				let base_row_size = (((ui.available_width() - 200.0) / content_width).floor() as usize).clamp(1, 5);
				let base_spacing = (ui.available_width() - content_width * (base_row_size) as f32 - 10.0 * (base_row_size - 1) as f32) / 2.0;	

				if !spells_full.is_empty() {
					ui.add_space(5.0);
					ui.add( egui::Separator::default().horizontal().shrink( base_spacing ));
					ui.add_space(10.0);
	
					ui.horizontal_top(|ui| {
						let row_size = if spells_full.len() < base_row_size {
							spells_full.len()
						} else {
							base_row_size.clone()
						};
						let spacing = (ui.available_width() - content_width * (row_size) as f32 - 10.0 * (row_size - 1) as f32) / 2.0;	
						let mut row_pos = 0;
	
						ui.add_space(spacing);
						egui::Grid::new("grid_spell_full_match")
							.min_col_width(content_width)
							.max_col_width(content_width)
							.spacing(egui::vec2(10.0, 10.0))
							.show(ui, |ui| {
							for spell in spells_full {
								spell.show(ui, ctx);
	
								row_pos += 1;
								if row_pos >= row_size {
									row_pos = 0;
									ui.end_row();
								}
							};
						});
					});
				}

				if !spells_partial.is_empty() {

					ui.add_space(10.0);
					ui.add( egui::Separator::default().horizontal().shrink( base_spacing ));
					ui.add_space(10.0);
	
					ui.horizontal_top(|ui| {
						let row_size = if spells_partial.len() <= base_row_size{
							spells_partial.len()
						} else {
							base_row_size.clone()
						};
						let spacing = (ui.available_width() - content_width * (row_size) as f32 - 10.0 * (row_size - 1) as f32) / 2.0;	
						let mut row_pos = 0;

						ui.add_space(spacing);
						egui::Grid::new("grid_spell_partial_match")
							.min_col_width(content_width)
							.max_col_width(content_width)
							.spacing(egui::vec2(10.0, 10.0))
							.show(ui, |ui| {
							for spell in spells_partial {
								spell.show(ui, ctx);
	
								row_pos += 1;
								if row_pos >= row_size {
									row_pos = 0;
									ui.end_row();
								}
							};
						});
						// ui.add_space(spacing);
					});
				}

			});
			});
		}
		Page::Character => {
			menubar::upper(parent, ctx, frame);

			egui::CentralPanel::default().show(ctx, |ui| {
				ui.label("Character here");
			});
		},
	}

}