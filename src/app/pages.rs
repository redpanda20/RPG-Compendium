
use super::icon;
use super::defines;

use super::spells;
use super::character;

use super::App;
use super::popups;
use super::loader;

pub mod compendium;
pub use compendium::show_spells;

pub mod show_characters;
pub use show_characters::show_all_characters;
pub use show_characters::show_character;

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Page {
	Home,
	#[serde(skip)]
	Compendium(spells::SpellType),
	AllCharacters,
	CharacterSheet(character::CharacterSheetDetails)
}
pub fn show_home(parent: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	let booklet = icon::from_png_responsive(
		defines::COMPENDIUM_LIGHT.to_vec(),
		defines::COMPENDIUM.to_vec(),
		ctx)
		.get(ctx);
	let characters = icon::from_png_responsive(
		defines::CHARACTERS_LIGHT.to_vec(),
		defines::CHARACTERS.to_vec(),
		ctx)
		.get(ctx);

	let mut open_compendium = false;
	let mut open_characters = false;

	egui::CentralPanel::default().show(ctx, |ui| {
		if ui.available_width() > 500.0 {

			ui.columns(3, |column| {

				column[0].add_space(100.0);

				open_compendium = column[0].add(
					egui::Button::image_and_text(
						booklet.0,
						booklet.1,
						egui::RichText::new("Compendium").size(16.0))	
							.min_size(egui::vec2(column[0].available_width(), 30.0))
				).clicked();

				column[0].add_space(50.0);

				open_characters = column[0].add(
					egui::Button::image_and_text(
						characters.0,
						characters.1,
						egui::RichText::new("Characters").size(16.0))	
							.min_size(egui::vec2(column[0].available_width(), 30.0))
				).clicked();

			});
		} else {		
			open_compendium =  ui.add(
				egui::Button::image_and_text(booklet.0, booklet.1, "Compendium")	
			).clicked();

			open_characters =  ui.add(
				egui::Button::image_and_text(characters.0, characters.1, "Characters")	
			).clicked();

		}
	});

	if open_compendium {
		parent.current_page = Page::Compendium(spells::SpellType::None);
	}
	if open_characters {
		parent.current_page = Page::AllCharacters;
	}
}
