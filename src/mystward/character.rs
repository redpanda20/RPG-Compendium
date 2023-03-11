use crate::resources::{image, icon, defines, loader};

pub mod racial_archetypes;
pub mod attributes;
pub mod items;
pub mod traits;

pub use racial_archetypes::*;
pub use attributes::Attribute;
pub use traits::Trait;

use super::spells;

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Character {
	pub character_info: CharacterInfo,
	pub archetype: RacialArchetype,
	pub attributes: Vec<(Attribute, u8)>,
	pub traits: Vec<Trait>,
	pub items: items::ItemList,
	pub notes: String,
}
#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CharacterInfo {
	pub name: String,
	pub biography: String,
	pub appearance: String,
	
	#[serde(skip)]
	profile_image: image::OptionalImage,	
	pub image_storage: Option<Vec<u8>>,
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CharacterSheetDetails {
	name: Option<String>,
	biography: Option<String>,
	appearance: Option<String>
}
impl CharacterSheetDetails {
	pub fn new() -> CharacterSheetDetails {
		CharacterSheetDetails { name: None, biography: None, appearance: None }
	}
}

impl Character {
	pub fn new(name: String, archetype: RacialArchetype) -> Self {

		let mut attributes: Vec<(Attribute, u8)> = attributes::default().to_vec();
		for (attribute, quanity) in attributes::from_archetype(&archetype) {
			let pos = attributes.iter().position(|(att, _)| att == &attribute);
			match pos {
				Some(index) => attributes[index].1 = attributes[index].1 + quanity,
				None => attributes.push((attribute, quanity)),
			}
		}

		let traits = traits::from_archetype(&archetype);

		Self {
			character_info: CharacterInfo {
				name,
				biography: String::new(),
				appearance: String::new(),
				profile_image: image::OptionalImage::default(),
				image_storage: None
			},
			archetype,
			attributes,
			traits,
			items: items::load_requisition_items(),
			notes: String::new() }
	}

	pub fn show(&mut self, loader: &mut loader::Loader, details: &mut CharacterSheetDetails, ui: &mut egui::Ui, ctx: &egui::Context, width: f32) {
		ui.vertical(|ui| {
			ui.set_width(width);
			ui.add_space(10.0);

			self.show_biography(loader, details, ui, ctx, width);

			ui.add_space(20.0);
		
			egui::Frame::menu(ui.style())
				.inner_margin(egui::Margin::symmetric(20.0, 4.0))
				.show(ui, |ui| {
				if width < 800.0 {
					self.show_attributes(ui);
					self.show_traits(ui);
					self.items.show(ui);
				} else {
					ui.columns(3, |column| {
						self.show_attributes(&mut column[0]);
						self.show_traits(&mut column[1]);
						self.items.show(&mut column[2]);
					});
				}
			});			
		});
	}

	fn show_biography(&mut self, loader: &mut loader::Loader, details: &mut CharacterSheetDetails, ui: &mut egui::Ui, ctx: &egui::Context, width: f32) {
		let edit_icon = icon::Icon::from_svg_responsive(defines::EDIT.to_vec(), ctx).get(ctx);

		let vertical_layout = width < 800.0;
		ui.style_mut().visuals.widgets.inactive.weak_bg_fill = ui.style().visuals.window_fill();

		egui::Frame::menu(ui.style()).show(ui, |ui| {
		ui.set_width(width);

		ui.vertical(|ui| {
		// Name
			ui.horizontal(|ui| {
				if ui.add(
					egui::ImageButton::new(edit_icon.0, edit_icon.1)
				).clicked() {
					details.name = Some(self.character_info.name.clone())
				};

				let mut char_name = self.character_info.name.as_str();
				let text: &mut dyn egui::TextBuffer = match &mut details.name {
					Some(text) => text, // Editable text
					None => &mut char_name, // Uneditable text
				};

				if egui::TextEdit::singleline(text)
					.font(egui::FontId::proportional(36.0))
					.show(ui)
					.response
				.lost_focus() {
					if let Some(name) = &details.name {
						self.character_info.name = name.clone();
					}
					details.name = None;
				};
			});

	// Archetype
			ui.horizontal(|ui| {
				ui.add_space(12.0);
				ui.label(egui::RichText::new(self.archetype.to_string()).size(18.0));
			});

			ui.separator()
		});
		ui.columns(
			match vertical_layout { true => 1, false => 2 },
			|columns| {

			columns[0].vertical(|ui| {

		// Image
				if vertical_layout {
					self.get_picture(egui::vec2(150.0, 150.0), loader, ctx, ui);
				}

				ui.add_space(20.0);

		// Biography
				ui.horizontal(|ui| {
					if ui.add(
						egui::ImageButton::new(edit_icon.0, edit_icon.1)
					).clicked() {
						details.biography = Some(self.character_info.biography.clone())
					};
					ui.label(egui::RichText::new("Biography").size(24.0));
				});
				ui.horizontal(|ui| {
					let mut biography = self.character_info.biography.as_str();
					let text: &mut dyn egui::TextBuffer = match &mut details.biography {
						Some(text) => text, // Editable text
						None => &mut biography, // Uneditable text
					};
					ui.add_space(10.0);
					if egui::TextEdit::multiline(text)
						.hint_text("What is your characters background?")
						.desired_width(ui.available_width())
						.show(ui)
						.response
					.lost_focus() {
						if let Some(text) = &details.biography {
							self.character_info.biography = text.clone();
						}
						details.biography = None;
					};
				});
				
		// Appearance
				ui.horizontal(|ui| {
					if ui.add(
						egui::ImageButton::new(edit_icon.0, edit_icon.1)
					).clicked() {
						details.appearance = Some(self.character_info.appearance.clone())
					};
					ui.label(egui::RichText::new("Biography").size(24.0));
				});
				ui.horizontal(|ui| {
					let mut appearance = self.character_info.appearance.as_str();
					let text: &mut dyn egui::TextBuffer = match &mut details.appearance {
						Some(text) => text, // Editable text
						None => &mut appearance, // Uneditable text
					};	
					ui.add_space(10.0);
					if egui::TextEdit::multiline(text)
						.hint_text("What does your character look like?")
						.desired_width(ui.available_width())
						.show(ui)
						.response
					.lost_focus() {
						if let Some(text) = &details.appearance {
							self.character_info.appearance = text.clone();
						}
						details.appearance = None;
					};
				});

				ui.label(egui::RichText::new("Player Notes").size(24.0));
				ui.horizontal(|ui| {
					ui.add_space(10.0);
					ui.add(
						egui::TextEdit::multiline(&mut self.notes)
						.hint_text("Character or session notes")
						.desired_width(ui.available_width())
						.desired_rows(8)
					);
				});
			});

			if !vertical_layout {
				self.get_picture(egui::vec2(350.0, 350.0), loader, ctx, &mut columns[1]);
			}

		});
		});
	}

	fn show_attributes(&self, ui: &mut egui::Ui) {
		ui.vertical(|ui| {
			ui.vertical_centered(|ui| {
				ui.label(egui::RichText::new("Attributes").size(24.0));
			});
			ui.separator();

			if let Some((_, quantity)) = &self.attributes.iter().find(|(att, _)| att == &Attribute::Unused) {
				ui.label( egui::RichText::new(format!("{}: {}", Attribute::Unused.to_string(), quantity)).size(16.0) );
				ui.add_space(10.0);
			}

			for (attribute, quantity) in &self.attributes {
				if attribute == &Attribute::Unused {continue;}
				ui.label( egui::RichText::new(format!("{}: {}", attribute.to_string(), quantity)).size(16.0) );
			}
		});
	}

	fn show_traits(&self, ui: &mut egui::Ui) {
		ui.vertical(|ui| {
			ui.vertical_centered(|ui| {
				ui.label(egui::RichText::new("Traits").size(24.0));
			});
			ui.separator();

			for char_trait in &self.traits {
				ui.label( egui::RichText::new(char_trait.title.clone()).size(20.0) );
				ui.label( egui::RichText::new(char_trait.text.clone()).size(12.0) );
				ui.add_space(4.0);
			}
		});
	}

	pub fn update_picture(&mut self, ctx: &egui::Context, file_raw: Vec<u8>) {
		self.character_info.profile_image.load_image_from_raw(ctx, file_raw.clone());
		self.character_info.image_storage = Some(file_raw.clone());
	}

	fn get_picture(&mut self, size: egui::Vec2, loader: &mut loader::Loader, ctx: &egui::Context, ui: &mut egui::Ui) {
		let (id, _) = &self.character_info.profile_image.get(ctx);

		ui.vertical_centered(|ui| {
			if ui.add(
				egui::ImageButton::new(*id, size)
			).double_clicked() {
				loader.file_dialog(loader::FileUsage::CharacterPicture)
			};
		});
	}
}