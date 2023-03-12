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
impl Default for CharacterSheetDetails {
	fn default() -> Self {
		Self { name: None, biography: None, appearance: None }
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
					self.show_items(ui);
				} else {
					ui.columns(3, |column| {
						self.show_attributes(&mut column[0]);
						self.show_traits(&mut column[1]);
						self.show_items(&mut column[2]);
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

		ui.with_layout(
			match vertical_layout {
				true => egui::Layout::top_down(egui::Align::Min),
				false => egui::Layout::left_to_right(egui::Align::Min),
			},
			|ui| {

				if vertical_layout {
					ui.vertical_centered(|ui| {
						self.get_picture(loader, ctx, ui);
					});
				} else {
					ui.horizontal_centered(|ui| {
						self.get_picture(loader, ctx, ui);
					});
				} 
				
				ui.add_space(10.0);

				ui.vertical(|ui| {

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
						ui.label(egui::RichText::new("Appearance").size(24.0));
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
			});
		});
	}

	fn show_attributes(&mut self, ui: &mut egui::Ui) {
		let mut temp_attributes = self.attributes.clone();
		ui.vertical(|ui| {
			ui.vertical_centered(|ui| {
				ui.label(egui::RichText::new("Attributes").size(24.0));
			});	
			ui.separator();

			let mut unused_quantity: Option<u8> = None;
			if let Some((_, quantity)) = &self.attributes.iter().find(|(att, _)| att == &Attribute::Unused) {
				unused_quantity = Some(*quantity);
			}

			egui::Grid::new("Attribute grid")
			.show(ui, |ui| {
				for (attribute, quantity) in &mut temp_attributes {
					if attribute == &Attribute::Unused { continue; }
					if ui.add_enabled(
						*quantity > 0,
						egui::Button::new("-")
					).clicked() {
						*quantity -= 1;
						if let Some(unused) = &mut unused_quantity {
							*unused += 1;
						} else { unused_quantity = Some(1) }
					};
					if unused_quantity.is_some() && unused_quantity.unwrap() > 0 {
						if ui.add_enabled(
							*quantity < 5,
							egui::Button::new("+")
						).clicked() {
							if let Some(unused) = &mut unused_quantity {
								if *unused > 0 {
									*quantity += 1;
									*unused -= 1;	
								}
							}
						};	
					} else {
						ui.label("");	// Dummy space so grid doesn't move
					} 
					ui.label( egui::RichText::new(attribute.to_string()).size(16.0) );
					let mut count = String::new();
					for _ in 0..*quantity {
						count += "|";
					}
					ui.label(count);
					ui.end_row();
				};
			});

			if let Some(quantity) = unused_quantity {
				if quantity > 0 {
					ui.add_space(10.0);
					ui.label(
						egui::RichText::new(format!("{}: {}", Attribute::Unused.to_string(), unused_quantity.unwrap()))
						.size(16.0)
						.color(ui.style().visuals.warn_fg_color));
				}
			}

			if let Some(new_quantity) = unused_quantity {
				for (att, quantity) in &mut temp_attributes {
					if att == &Attribute::Unused {
						*quantity = new_quantity;
						break;
					}
				}
			}
		});
		self.attributes = temp_attributes;
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

	fn show_items(&self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.add_space(60.0);
			ui.with_layout(
				egui::Layout::right_to_left(egui::Align::Center),
				|ui| {
					ui.add_space(10.0);
					ui.add_sized(egui::vec2(50.0, 0.0), egui::Button::new("Update"));
					ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Items").size(24.0));
					});
			});
		});
		ui.separator();
		self.items.show(ui);
	}

	pub fn update_picture(&mut self, ctx: &egui::Context, file_raw: Vec<u8>) {
		self.character_info.profile_image.update(ctx, file_raw.clone());
		self.character_info.image_storage = Some(file_raw.clone());
	}

	pub fn get_picture(&mut self, loader: &mut loader::Loader, ctx: &egui::Context, ui: &mut egui::Ui) {
		let (id, size) = &self.character_info.profile_image.get(ctx);

		if ui.add(
			egui::ImageButton::new(*id, *size)
		).clicked() {
			loader.file_dialog(loader::FileUsage::CharacterPicture)
		};
	}

	pub fn get_picture_static(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
		let (id, size) = &self.character_info.profile_image.get_thumbnail(ctx);

		ui.image(*id, *size);
	}
}