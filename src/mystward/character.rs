use crate::resources::{image, icon, defines};

pub mod racial_archetypes;
pub mod attributes;
pub mod items;
pub mod traits;
pub mod advances;

pub use racial_archetypes::*;
pub use attributes::Attribute;
pub use traits::Trait;
pub use advances::Advance;

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
	appearance: Option<String>,
	item_selection: Option<items::ItemList>
}
impl Default for CharacterSheetDetails {
	fn default() -> Self {
		Self { name: None, biography: None, appearance: None, item_selection: None }
	}
}
pub enum Request {
	None,
	UpdatePicture,
	PopupLevelUp
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
			items: items::all_requisition_items(),
			notes: String::new() }
	}

	pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, details: &mut CharacterSheetDetails, width: f32) -> Request {
		let mut request = Request::None;
		ui.vertical(|ui| {
			ui.set_width(width);
			ui.add_space(10.0);

			let bio_request = self.show_biography(ui, ctx, details, width);
			match bio_request {
				Request::None => (),
				Request::UpdatePicture => request = Request::UpdatePicture,
				Request::PopupLevelUp => request = Request::PopupLevelUp,
			}

			ui.add_space(20.0);
		
			egui::Frame::menu(ui.style())
				.inner_margin(egui::Margin::symmetric(20.0, 4.0))
				.show(ui, |ui| {
				if width < 800.0 {
					self.show_attributes(ui);
					self.show_traits(ui);
					self.show_items(ui, details);
				} else {
					ui.columns(3, |column| {
						self.show_attributes(&mut column[0]);
						self.show_traits(&mut column[1]);
						self.show_items(&mut column[2], details);
					});
				}
			});			
		});
		return request;
	}

	fn show_biography(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, details: &mut CharacterSheetDetails, width: f32) -> Request {
		let mut request = Request::None;
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

				ui.with_layout(
					egui::Layout::right_to_left(egui::Align::Center),
					|ui| {
						let (id, size) = icon::from_png_responsive(
							defines::DOUBLE_UP_LIGHT.to_vec(),
							defines::DOUBLE_UP.to_vec(),
							ctx).get(ctx);
						if ui.add(
							egui::Button::image_and_text(
								id,
								size,
								"Level Up")
						).clicked() {
							request = Request::PopupLevelUp;
						};
					})
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

				let mut update_picture = false;
				if vertical_layout {
					ui.vertical_centered(|ui| {
						update_picture = self.get_picture(ctx, ui);
					});
				} else {
					ui.horizontal_centered(|ui| {
						update_picture = self.get_picture(ctx, ui);
					});
				} 
				if update_picture {
					request = Request::UpdatePicture;
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
		return request;
	}

	fn show_attributes(&mut self, ui: &mut egui::Ui) {
		let mut temp_attributes = self.attributes.clone();
		ui.vertical(|ui| {
			ui.vertical_centered(|ui| {
				ui.label(egui::RichText::new("Attributes").size(24.0));
			});	
			ui.separator();

			let mut unused_quantity: u8 = 0;
			if let Some((_, quantity)) = &self.attributes.iter().find(|(att, _)| att == &Attribute::Unused) {
				unused_quantity = *quantity;
			}

			fn draw_box(ui: &mut egui::Ui, alt_fill: Option<egui::Color32>, react: bool, react_fill: Option<egui::Color32>) -> bool {
				let (rect, response) = ui.allocate_at_least(egui::vec2(24.0, 16.0), egui::Sense::click());
				let style = match react {
					true => ui.style().as_ref().interact(&response).to_owned(),
					false => {
						let mut new_style = ui.style().visuals.widgets.hovered;
						if let Some(fill) = react_fill {
							if response.hovered() {
								new_style.bg_fill = fill;
							}
						}
						new_style
					},
				};
				ui.painter().rect(
					rect,
					style.rounding,
					match alt_fill {
						Some(fill) => fill,
						None => style.bg_fill,
					},
					style.bg_stroke);
				response.clicked()
			}

			let item_weight = match self.archetype {
				RacialArchetype::Byvine(ByvineClass::Goliath) => match self.items.item_weight_custom([0, 1, 1]) {
					0 => 0,
					num => num - 1,
				},
				RacialArchetype::Wyvren(_) => self.items.item_weight(),
				_ => match self.items.item_weight() {
					0 => 0,
					num => num - 1,
				},
			} as u8;
	

			ui.style_mut().spacing.item_spacing.y = 8.0;
			ui.columns(2, |column| {
				for (attribute, quantity) in &mut temp_attributes {
					if attribute == &Attribute::Unused {
						continue;
					}
					column[0].label( egui::RichText::new(attribute.to_string()).size(16.0) );
					column[1].horizontal(|ui| {
						match attribute {
							Attribute::Athletics => {
								for _ in 0..item_weight as u8 {
									draw_box(ui, Some(egui::Color32::DARK_RED), false, None);
								}
								if *quantity > item_weight as u8 + 1 {
									for _ in 0..*quantity - 1 - self.items.item_weight() as u8 {
										draw_box(ui, None, false, None);
									}
								}
								if *quantity > item_weight as u8 {
									if draw_box(ui, None, false, Some(egui::Color32::RED)) {
										*quantity -= 1;
										unused_quantity += 1;				
									}
								}								
							},
							_ => {
								if *quantity > 0 {
									for _ in 0..*quantity-1 {
										draw_box(ui, None, false, None);
									}
									if draw_box(ui, None, false, Some(egui::Color32::RED)) {
										*quantity -= 1;
										unused_quantity += 1;				
									}
								}
							},
						}
						if unused_quantity > 0 && *quantity < 5 {
							if draw_box(ui, None, true, None) {
								*quantity += 1;
								unused_quantity -= 1;		
							}
						}
						// Keep values inline if nothing would be rendered
						if *quantity == 0 {
							let (_, _) = ui.allocate_at_least(egui::vec2(24.0, 16.0), egui::Sense::hover());
						}
					});
				};
			});

			if unused_quantity > 0 {
				ui.add_space(10.0);
				ui.label(
					egui::RichText::new(format!("{}: {}", Attribute::Unused.to_string(), unused_quantity))
					.size(16.0)
					.color(ui.style().visuals.warn_fg_color));
			}

			for (att, quantity) in &mut temp_attributes {
				if att == &Attribute::Unused {
					*quantity = unused_quantity;
					break;
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

	fn show_items(&mut self, ui: &mut egui::Ui, details: &mut CharacterSheetDetails) {
		if self.items.clear_unused() {
			details.item_selection = None
		}
		let mut selected_item : Option<items::Item> = None;

		let small_item_header = if self.items.small_item_count() == 0 {
			"No Small Items"
		} else {
			"Small Items"
		};
		ui.horizontal(|ui| {
			ui.add_space(30.0);
			ui.with_layout(
				egui::Layout::right_to_left(egui::Align::Max),
				|ui| {
					if self.items.small_item_count() < 4 && details.item_selection.is_none() {
						if ui.add_sized(egui::vec2(30.0, 20.0), egui::Button::new("Add")).clicked() {
							details.item_selection = Some(items::small_requisition_items());
						}
					} else {
						ui.add_space(30.0);
					}
					ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new(small_item_header).size(24.0));
					});
			});	
		});
		ui.separator();

		ui.vertical(|ui| {
			self.items.show_items(ui, items::Weight::Small);
			if let Some(items) = &mut details.item_selection {
				if let Some(item) = items.show_items_selectable(ui, items::Weight::Small) {
					selected_item = Some(item);
				};
			}
		});
		
		// Items with weight
		let mut max_weight = 0;
		if let Some((_, quantity)) = &self.attributes.iter().find(|(att, _)| att == &Attribute::Athletics) {
			max_weight += *quantity as usize;
		}

		let item_weight = match self.archetype {
			RacialArchetype::Byvine(ByvineClass::Goliath) => match self.items.item_weight_custom([0, 1, 1]) {
				0 => 0,
				num => num - 1,
			},
			RacialArchetype::Wyvren(_) => self.items.item_weight(),
			_ => match self.items.item_weight() {
				0 => 0,
				num => num - 1,
			},
		};


		ui.horizontal(|ui| {
			ui.add_space(30.0);
			ui.with_layout(
				egui::Layout::right_to_left(egui::Align::Max),
				|ui| {
					if details.item_selection.is_none() && item_weight + 1 <= max_weight {
						let mut valid_items = items::normal_requisition_items();
						if match self.archetype {
							RacialArchetype::Byvine(ByvineClass::Goliath) => true,
							RacialArchetype::Wyvren(_) => false,
							_ => item_weight + 2 <= max_weight,
						} {
							valid_items.add_item_list(items::heavy_requisition_items());
						}
						if ui.button("Add").clicked() {
							details.item_selection = Some(valid_items);
						}
					} else {
						ui.add_space(30.0);
					}
					ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Items").size(24.0));
					});
			});	
		});
		ui.separator();

		ui.vertical(|ui| {
			self.items.show_items(ui, items::Weight::Normal);
			self.items.show_items(ui, items::Weight::Heavy);
			if let Some(items) = &mut details.item_selection {
				if let Some(item) = items.show_items_selectable(ui, items::Weight::Normal) {
					selected_item = Some(item);
				};
				if let Some(item) = items.show_items_selectable(ui, items::Weight::Heavy) {
					selected_item = Some(item);
				};
			}
		});

		// If item chosen then update
		if let Some(item) = selected_item {
			self.items.add_item(item);
			details.item_selection = None
		}
	}

	pub fn update_picture(&mut self, ctx: &egui::Context, file_raw: Vec<u8>) {
		self.character_info.profile_image.update(ctx, file_raw.clone());
		self.character_info.image_storage = Some(file_raw.clone());
	}

	pub fn get_picture(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> bool {
		let (id, size) = &self.character_info.profile_image.get(ctx);

		ui.add(
			egui::ImageButton::new(*id, *size)
		)
		.clicked()
	}

	pub fn get_picture_static(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
		let (id, size) = &self.character_info.profile_image.get_thumbnail(ctx);

		ui.image(*id, *size);
	}
}