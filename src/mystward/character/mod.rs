use crate::resources::{image, icon, defines, loader};

pub mod racial_archetypes;
pub mod items;

pub use racial_archetypes::*;
use items::*;

use super::spells::SpellType;

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Character {
	pub character_info: CharacterInfo,
	pub archetype: RacialArchetype,
	pub attributes: Vec<(Attribute, u8)>,
	pub items: Vec<Item>,
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
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Attribute {
	Unused,
	Athletics,
	Melee,
	Marksmanship,
	Stealth,
	FirstAid,
	Diplomacy(Race),
	Magic(SpellType),
	Lore(Lore),
	Blacksmith,
	Explosives,
	Engineering,
	Survivalist,
}
impl ToString for Attribute {
    fn to_string(&self) -> String {
        match self {
			Attribute::Unused => "Unused",
            Attribute::Athletics => "Athletics",
            Attribute::Melee => "Melee",
            Attribute::Marksmanship => "Marksmanship",
            Attribute::Stealth => "Stealth",
            Attribute::FirstAid => "First Aid",
            Attribute::Diplomacy(race) => match race {
                Race::Cerudant => "Diplomacy (The Cerudant Council)",
                Race::InkurtAnami => "Diplomacy (Inkurt Anami)",
                Race::Rova => "Diplomacy (The Kingdom of Rova)",
            },
            Attribute::Magic(magic_type) => match magic_type {
                SpellType::None => "",
                SpellType::Arcane(_) => "Arcane Magic",
                SpellType::Fae(patron) => match patron {
                    super::spells::FaePatron::Generic => "Fae Magic",
                    super::spells::FaePatron::Pixie => "Fae Magic (Pixie)",
                    super::spells::FaePatron::Sylviel => "Fae Magic (Sylviel)",
                    super::spells::FaePatron::ForgeSprite => "Fae Magic (Forge Sprite)",
                },
            },
            Attribute::Lore(lore) => match lore {
                Lore::Fables => "Lore (Fables)",
                Lore::Beasts => "Lore (Beasts)",
                Lore::Plants => "Lore (Plants)",
                Lore::Commerce => "Lore (Commerce)",
                Lore::Art => "Lore (Art)",
                Lore::Architecture => "Lore (Architecture)",
                Lore::Geology => "Lore (Geology)",
                Lore::History => "Lore (History)",
            },
            Attribute::Blacksmith => "Blacksmithing",
            Attribute::Explosives => "Explosives",
            Attribute::Engineering => "Engineering",
            Attribute::Survivalist => "Survivalist",
        }.to_owned()
    }
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Race {
	Cerudant,
	InkurtAnami,
	Rova
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Lore {
	Fables,
	Beasts,
	Plants,
	Commerce,
	Art,
	Architecture,
	Geology,
	History
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
	pub fn new(name: String, racial_archetype: RacialArchetype) -> Self {
		let attributes = vec![
			(Attribute::Unused, 6),
			(Attribute::Athletics, 1),
			(Attribute::Melee, 0),
			(Attribute::Marksmanship, 0),
			(Attribute::Stealth, 0),
			(Attribute::FirstAid, 0),	
		];
		Self {
			character_info: CharacterInfo {
				name, biography: String::new(),
				appearance: String::new(),
				profile_image: image::OptionalImage::default(),
				image_storage: None
			},
			archetype: racial_archetype,
			attributes,
			items: Vec::new(),
			notes: String::new() }
	}

	fn show_biography(&mut self, loader: &mut loader::Loader, details: &mut CharacterSheetDetails, ui: &mut egui::Ui, ctx: &egui::Context, vertical_layout: bool) {
		let edit_icon = icon::Icon::from_svg_responsive(defines::EDIT.to_vec(), ctx).get(ctx);
		ui.style_mut().visuals.widgets.inactive.weak_bg_fill = ui.style().visuals.window_fill();

		ui.horizontal(|ui| {
			ui.vertical(|ui| {

		// Name
				ui.horizontal(|ui| {
					if let Some(name) = &mut details.name {
						if ui.add(
							egui::TextEdit::singleline(name)
								.font(egui::FontId::proportional(36.0))
						).lost_focus() {
							if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
								self.character_info.name = name.clone();
							}
							details.name = None;
						}
					} else {
						ui.label(egui::RichText::new(&self.character_info.name).size(36.0));
					}
					if ui.add(
						egui::ImageButton::new(edit_icon.0, edit_icon.1)
					).clicked() {
						details.name = Some(self.character_info.name.clone())
					};
				});

		// Archetype
				ui.horizontal(|ui| {
					ui.add_space(12.0);
					ui.label(egui::RichText::new(self.archetype.to_string()).size(18.0));
				});

		// Image
				if vertical_layout {
					self.get_picture(egui::vec2(100.0, 100.0), loader, ctx, ui);
				}

				ui.add_space(20.0);

		// Biography
				let mut biography_open = false;
				ui.horizontal(|ui| {
					if ui.add(egui::ImageButton::new(edit_icon.0, edit_icon.1)).clicked() {
						biography_open = true;
					};
					ui.label(egui::RichText::new("Biography").size(24.0));
				});
				if let Some(biography) = &mut details.biography {
					if ui.text_edit_multiline(biography).lost_focus() {
						self.character_info.biography = biography.clone();
						details.biography = None;
					};
				} else {
					ui.add_enabled_ui(false, |ui| {
						ui.text_edit_multiline(&mut self.character_info.biography.clone());
					});	
				}
				if biography_open {
					details.biography = Some(self.character_info.biography.clone());
				}
				
		// Appearance
				let mut appearance_open = false;
				ui.horizontal(|ui| {
					if ui.add(egui::ImageButton::new(edit_icon.0, edit_icon.1)).clicked() {
						appearance_open = true;
					};
					ui.label(egui::RichText::new("Appearance").size(24.0));
				});
				if let Some(appearance) = &mut details.appearance {
					if ui.text_edit_multiline(appearance).lost_focus() {
						self.character_info.appearance = appearance.clone();
						details.appearance = None;
					};
				} else {
					ui.add_enabled_ui(false, |ui| {
						ui.text_edit_multiline(&mut self.character_info.appearance.clone());
					});	
				}
				if appearance_open {
					details.appearance = Some(self.character_info.appearance.clone())
				}


				ui.horizontal(|ui| {
					ui.add_space(38.0);
					ui.label(egui::RichText::new("Player Notes").size(24.0));
				});
				ui.text_edit_multiline(&mut self.notes);
			});

			if !vertical_layout {
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					self.get_picture(egui::vec2(300.0, 300.0), loader, ctx, ui);
				});
			}

		});
	}

	fn show_attributes(&self, ui: &mut egui::Ui) {
		ui.group(|ui| {
			ui.set_max_width(400.0);
			ui.vertical(|ui| {
				if let (Attribute::Unused, quantity) = self.attributes[0] {
					ui.label( egui::RichText::new(format!("{}: {}", Attribute::Unused.to_string(), quantity)).size(16.0) );
					ui.separator();
					ui.add_space(10.0);
				}

				for (attribute, quantity) in &self.attributes {
					if *attribute == Attribute::Unused {continue;}
					ui.label( egui::RichText::new(format!("{}: {}", attribute.to_string(), quantity)).size(16.0) );
				}
			});
		});
	}

	fn show_items(&self, ui: &mut egui::Ui) {
		ui.group(|ui| {
			ui.vertical(|ui| {
				for item in &self.items {
					item.show(ui)
				}
			});

		});

	}

	pub fn show(&mut self, loader: &mut loader::Loader, details: &mut CharacterSheetDetails, ui: &mut egui::Ui, ctx: &egui::Context, width: f32) {
		egui::Frame::menu(&ctx.style())
		.show(ui, |ui| {
			ui.set_width(width);
			ui.set_min_height(width);

			ui.vertical(|ui| {
				
				self.show_biography(loader, details, ui, ctx, width < 1000.0);

				ui.add_space(15.0);
				ui.separator();
				ui.add_space(25.0);
			
				if width < 800.0 {
					self.show_attributes(ui);
					self.show_items(ui);
				} else {
					ui.horizontal(|ui| {
						self.show_attributes(ui);
						self.show_items(ui);
					});
				}
			});
		});
	}

	pub fn update_picture(&mut self, ctx: &egui::Context, file_raw: Vec<u8>) {
		self.character_info.profile_image.load_image_from_raw(ctx, file_raw.clone());
		self.character_info.image_storage = Some(file_raw.clone());
	}

	fn get_picture(&mut self, size: egui::Vec2, loader: &mut loader::Loader, ctx: &egui::Context, ui: &mut egui::Ui) {
		let (id, _) = self.character_info.profile_image.get().unwrap_or_else( ||
			if let Some(file_raw) = &self.character_info.image_storage {
				self.update_picture(ctx, file_raw.to_vec());
				self.character_info.profile_image.get().unwrap()
			} else {
				icon::Icon::from_svg_responsive_with_size(
					defines::NO_IMAGE.to_vec(),
					[300, 300], ctx)
					.get(ctx)	
			}
		);
		if ui.add(
			egui::ImageButton::new(id, size)
		).clicked() {
			loader.file_dialog(loader::FileUsage::CharacterPicture)
		};
	}
}