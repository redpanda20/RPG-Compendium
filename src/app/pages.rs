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
						
							let ign = images::StaticSvg::new_single(String::from("Ignition"), images::IGNITION.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(ign.0, ign.1) ).clicked() {
								if ! active_concepts.insert(spells::ArcaneConcept::Ignition) {active_concepts.remove(&spells::ArcaneConcept::Ignition);} };

							let lif = images::StaticSvg::new_single(String::from("Life"), images::LIFE.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(lif.0, lif.1) ).clicked() {
								if ! active_concepts.insert(spells::ArcaneConcept::Life) {active_concepts.remove(&spells::ArcaneConcept::Life);} };

							let des = images::StaticSvg::new_single(String::from("Design"), images::DESIGN.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(des.0, des.1) ).clicked() {
								if ! active_concepts.insert(spells::ArcaneConcept::Design) {active_concepts.remove(&spells::ArcaneConcept::Design);} };

							let ast = images::StaticSvg::new_single(String::from("Astral"), images::ASTRAL.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(ast.0, ast.1) ).clicked() {
								if ! active_concepts.insert(spells::ArcaneConcept::Astral) {active_concepts.remove(&spells::ArcaneConcept::Astral);} };

							let fce = images::StaticSvg::new_single(String::from("Force"), images::FORCE.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(fce.0, fce.1) ).clicked() {
								if ! active_concepts.insert(spells::ArcaneConcept::Force) {active_concepts.remove(&spells::ArcaneConcept::Force);} };

							let wis = images::StaticSvg::new_single(String::from("Wisdom"), images::WISDOM.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(wis.0, wis.1) ).clicked() {
								if ! active_concepts.insert(spells::ArcaneConcept::Widsom) {active_concepts.remove(&spells::ArcaneConcept::Widsom);} };

							let ent = images::StaticSvg::new_single(String::from("Entropy"), images::ENTROPY.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(ent.0, ent.1) ).clicked() {
								if ! active_concepts.insert(spells::ArcaneConcept::Entropy) {active_concepts.remove(&spells::ArcaneConcept::Entropy);} };
						},
						spells::SpellType::Fae(_) => {
							let Page::Compendium(spells::SpellType::Fae(active_patron)) = &mut parent.current_page else { return };
							ui.add_space((ui.available_width() - 4.0 * (24.0 + 2.0 * ui.style().spacing.item_spacing.x + 2.0 * ui.style().spacing.button_padding.x)) / 2.0);
							
							let gen = images::StaticSvg::new(String::from("Generic"), images::HOME.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(gen.0, gen.1) ).clicked() {
								*active_patron = spells::FaePatron::Generic };

							let pix = images::StaticSvg::new_single(String::from("Pixie"), images::PIXIE.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(pix.0, pix.1) ).clicked() {
								*active_patron = spells::FaePatron::Pixie };

							let syv = images::StaticSvg::new_single(String::from("Sylviel"), images::SYLVIEL.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(syv.0, syv.1) ).clicked() {
								*active_patron = spells::FaePatron::Sylviel };

							let fge = images::StaticSvg::new_single(String::from("Forge Sprite"), images::FORGE_SPRITE.to_vec()).get(ctx);
							if ui.add( egui::ImageButton::new(fge.0, fge.1) ).clicked() {
								*active_patron = spells::FaePatron::ForgeSprite };
						},
						spells::SpellType::None => (),
						
					}
				});


				let filtered_spells: Vec<spells::Spell> = match selected_type {
					spells::SpellType::None => {
						all_spells
					}
					spells::SpellType::Arcane(selected_concepts) => {
						all_spells
							.into_iter()
							.filter(|spell| match &spell.spell_type {
								spells::SpellType::Arcane(concepts) => selected_concepts.is_empty() || !concepts.is_disjoint(&selected_concepts),
								_ => false})
							.collect()
					},
					spells::SpellType::Fae(selected_patron) => {
						all_spells
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
				// ui.add( egui::Separator::default());
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

				})

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