use super::images as images;
use super::menubar as menubar;
use super::spells as spells;

#[derive(PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Page {
	Home,
	Compendium(Selection),
	Character
}

#[derive(PartialEq, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Selection {
	#[serde(skip)]
	spell_type: Option<spells::SpellType>,
}
impl Default for Selection {
    fn default() -> Self {
        Self { spell_type: None }
    }
}
pub fn show(parent: &mut super::App, ctx: &egui::Context, frame: &mut eframe::Frame) {
	match parent.current_page {
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
					parent.current_page = Page::Compendium(Selection::default())
				};
			});
		},
		Page::Compendium(selection) => {
			menubar::upper(parent, ctx, frame);

			let all_spells = spells::get_all_spells(ctx);

			egui::CentralPanel::default().show(ctx, |ui| {
				ui.label("There shouldbe something here");
				ui.horizontal(|ui|{
					let spacing = (ui.available_width() - 500.0) / 2.0;
					ui.add_space(spacing);
					if ui.add_sized(
						egui::vec2(150.0, 50.0),
						egui::Button::new("Arcane")
					).clicked() {
						parent.current_page = Page::Compendium(
						Selection { spell_type:
							Some(spells::SpellType::Arcane([
								spells::ArcaneConcept::None,
								spells::ArcaneConcept::None,
								spells::ArcaneConcept::None]))
						})
					};
					if ui.add_sized(
						egui::vec2(150.0, 50.0),
						egui::Button::new("Fae")
					).clicked() {
						parent.current_page = Page::Compendium(
						Selection { spell_type:
							Some(spells::SpellType::Fae(spells::FaePatron::Generic))
						});
					};
					ui.add_space(spacing);

				});

				let filtered_spells: Vec<spells::Spell> = if let Some(selection_type) = selection.spell_type {
					match selection_type {
						spells::SpellType::Arcane(_) => {
							all_spells
								.into_iter()
								.filter(|spell| match spell.spell_type {
									spells::SpellType::Arcane(_) => true, _ => false})
								.collect()
						},
						spells::SpellType::Fae(_) => {
							all_spells
								.into_iter()
								.filter(|spell| match spell.spell_type {
									spells::SpellType::Fae(_) => true, _ => false})
								.collect()
						},
					}
				} else {
					all_spells
				};
	

				ui.horizontal_top(|ui| {
					let mut row_size = (ui.available_width() / 300.0).floor() as usize;
					if row_size > 5 {
						row_size = 5;
					}
					let spacing = (ui.available_width() - 300.0 * row_size.clone() as f32) / 2.0;	

					ui.add_space(spacing);

					egui::Grid::new("content_grid")
						.max_col_width(300.0)
						.max_col_width(300.0)
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
		}
		Page::Character => {
			menubar::upper(parent, ctx, frame);

			egui::CentralPanel::default().show(ctx, |ui| {
				ui.label("Character here");
			});
		},
	}

}