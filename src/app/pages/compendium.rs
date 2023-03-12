use super::*;

pub fn show_spells(parent: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	let Page::Compendium(selected_type) = parent.current_page.clone() else {
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
								ui.painter().hline(
									pos.x..=(pos.x + 24.0 + 2.0 * ui.style().spacing.button_padding.x),
									pos.y,
									ctx.style().visuals.selection.stroke);
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
								ui.painter().hline(
									pos.x..=(pos.x + 24.0 + 2.0 * ui.style().spacing.button_padding.x),
									pos.y,
									ctx.style().visuals.selection.stroke);
							}
						});
					}
				},
				spells::SpellType::None => (),
				spells::SpellType::Wild => (),
				
			}
		});

		let spell_match_full = match &selected_type {
			spells::SpellType::None => {
				spells::get_all_spells(ctx)
			},
			spells::SpellType::Wild => {
				spells::get_all_spells(ctx)
			},
			spells::SpellType::Arcane(selected_concepts) => {
				spells::get_all_spells(ctx)
					.into_iter()
					.filter(|spell| match &spell.spell_type {
						spells::SpellType::Arcane(concepts) => selected_concepts.is_empty() || concepts.is_subset(&selected_concepts),
						_ => false})
					.collect()
			},
			spells::SpellType::Fae(selected_patron) => {
				spells::get_all_spells(ctx)
					.into_iter()
					.filter(|spell| match spell.spell_type {
					spells::SpellType::Fae(patron) => *selected_patron == spells::FaePatron::Generic || patron == *selected_patron,
					_ => false })
					.collect()
			},
		};

		let spell_match_partial: Vec<spells::Spell> = match &selected_type {
			spells::SpellType::None => {
				Vec::new()
			},
			spells::SpellType::Wild => {
				Vec::new()
			},
			spells::SpellType::Arcane(selected_concepts) => {
				spells::get_all_spells(ctx)
					.into_iter()
					.filter(|spell| match &spell.spell_type {
						spells::SpellType::Arcane(concepts) => !concepts.is_disjoint(&selected_concepts),
						_ => false})
					.collect()
			},
			spells::SpellType::Fae(selected_patron) => {
				spells::get_all_spells(ctx)
					.into_iter()
					.filter(|spell| match spell.spell_type {
					spells::SpellType::Fae(patron) => *selected_patron != spells::FaePatron::Generic && patron == spells::FaePatron::Generic,
					_ => false })
					.collect()
			},
		};

		const MAX_ROW_SIZE: usize = 5;

		let content_width = (ui.available_width() / 6.0).clamp(200.0, 340.0);
		// let row_size = (((ui.available_width() - 200.0) / content_width).floor() as usize).clamp(1, MAX_ROW_SIZE);
		let spacing = (ui.available_width() - content_width * MAX_ROW_SIZE as f32 - 10.0 * (MAX_ROW_SIZE - 1) as f32) / 2.0;	

		ui.add_space(5.0);
		ui.add( egui::Separator::default().horizontal().shrink( spacing ));
		ui.add_space(10.0);

		ui.horizontal_top(|ui| {
			let mut row_size = (((ui.available_width() - 200.0) / content_width).floor() as usize)
				.clamp(1, MAX_ROW_SIZE);
			if !spell_match_full.is_empty() && spell_match_full.len() < MAX_ROW_SIZE {
				row_size = row_size.clamp(1, spell_match_full.len());
			}
			ui.add_space((ui.available_width() - content_width * row_size as f32 - 10.0 * (row_size - 1) as f32) / 2.0);

			egui::Grid::new("full match grid")
				.max_col_width(content_width)
				.spacing(egui::vec2(10.0, 10.0))
				.show(ui, |ui| {
				let mut row_length = 0;
				for spell in spell_match_full {
					spell.show(ui, ctx, content_width);

					row_length += 1;
					if row_length >= row_size {
						row_length = 0;
						ui.end_row();
					}
				}
			});
		});

		if !spell_match_partial.is_empty() {
			ui.add_space(10.0);
			ui.add( egui::Separator::default().horizontal().shrink( spacing ));
			ui.add_space(10.0);
	
			ui.horizontal_top(|ui| {
				let mut row_size = (((ui.available_width() - 200.0) / content_width).floor() as usize)
					.clamp(1, MAX_ROW_SIZE);
				if spell_match_partial.len() < MAX_ROW_SIZE {
					row_size = row_size.clamp(1, spell_match_partial.len());
				}
				ui.add_space((ui.available_width() - content_width * row_size as f32 - 10.0 * (row_size - 1) as f32) / 2.0);
	
				egui::Grid::new("partial match grid")
					.max_col_width(content_width)
					.spacing(egui::vec2(10.0, 10.0))
					.show(ui, |ui| {
					let mut row_length = 0;
					for spell in spell_match_partial {
						spell.show(ui, ctx, content_width);
	
						row_length += 1;
						if row_length >= row_size {
							row_length = 0;
							ui.end_row();
						}
					}
				});
			});
		}
	
	});
	});
}