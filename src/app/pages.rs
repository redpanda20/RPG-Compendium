
use super::images as images;
use super::menubar as menubar;

#[derive(PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Page {
	Home,
	Compendium,
	Character
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
					parent.current_page = Page::Compendium
				};
			});
		},
		Page::Compendium => {
			menubar::upper(parent, ctx, frame);

			egui::TopBottomPanel::bottom("References").show(ctx, |ui| {
				ui.label("Built by Alexandra Stephens");
				ui.hyperlink_to("Built using eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
			});

			egui::CentralPanel::default().show(ctx, |ui| {
				ui.label("There should be something here!");
				ui.text_edit_multiline(&mut parent.text);
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