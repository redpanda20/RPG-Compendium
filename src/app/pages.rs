
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
