use crate::resources::{defines, icon};

#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Item {
	name: String,
	flavour_text: Option<String>,
	weight: Weight,
	effect: Option<String>,
	equipped: bool,
}
#[derive(Clone, PartialEq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Weight {
	Small,
	Normal,
	Heavy,
}
impl Default for Item {
    fn default() -> Self {
        Self {
			name: String::from(""),
			flavour_text: None,
			weight: Weight::Normal,
			effect: None,
			equipped: false,
		}
    }
}
impl Item {
	pub fn show(&mut self, ui: &mut egui::Ui) {	
		let rect = ui.horizontal(|ui| {
			ui.vertical(|ui| {
				let text = match self.weight == Weight::Heavy {
					true => self.name.to_owned() + " (Heavy)",
					false => self.name.to_owned(),
				};
				let mut title = text.as_str();
				egui::TextEdit::singleline(&mut title)
					.font(egui::FontId::proportional(16.0))
					.show(ui);

				if let Some(text) = &self.effect {
					let mut effect = text.as_str();
					egui::TextEdit::multiline(&mut effect)
						.desired_rows(1)
						.show(ui);
				}
				if let Some(text) = &self.flavour_text {
					ui.label(egui::RichText::new(text).italics());
				}
				});
			ui.with_layout(
				egui::Layout::right_to_left(egui::Align::Min),
				|ui| {
					let (id, size) = icon::from_png_responsive(
						defines::CLOSE_LIGHT.to_vec(),
						defines::CLOSE.to_vec(),
						ui.ctx())
						.get(ui.ctx());
					if ui.add(egui::ImageButton::new(id, size)).clicked() {
						self.equipped = false;
					};
				});
		}).response.rect;
		ui.painter().rect_stroke(
			rect,
			ui.style().visuals.widgets.noninteractive.rounding,
			ui.style().visuals.widgets.noninteractive.bg_stroke);
	}
	pub fn show_selectable(&mut self, ui: &mut egui::Ui) -> bool {	
		let mut height_estimate = 16.0 + 12.0;
		if let Some(text) = &self.effect {
			height_estimate += 13.0 * text.lines().count() as f32
		}		
		if self.flavour_text.is_some() {
			height_estimate += 13.0;
		}
		
		let (rect, response) = ui.allocate_at_least(
			egui::vec2(ui.available_width(), height_estimate),
			egui::Sense::click());
		let style = ui.style().as_ref().interact(&response);
		ui.painter().rect_stroke(
			rect,
			style.rounding,
			style.bg_stroke);
		ui.allocate_ui_at_rect(rect, |ui| {
			ui.set_min_height(rect.height());
			ui.horizontal_centered(|ui| {
				ui.add_space(6.0);
				ui.vertical(|ui| {
					match self.weight == Weight::Heavy {
						true => ui.label(egui::RichText::new(self.name.to_owned() + " (Heavy)").size(16.0)),
						false => ui.label(egui::RichText::new(&self.name).size(16.0)),
					};
					if let Some(text) = &self.effect {
						ui.label(text);
					}
					if let Some(text) = &self.flavour_text {
						ui.label(egui::RichText::new(text).italics());
					}
				});	
			});
		});
		response.clicked()
	}
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ItemList {
	item_list: Vec<Item>
}
impl ItemList {
	pub fn add_item(&mut self, item: Item) {
		self.item_list.push(item);
	}
	pub fn add_item_list(&mut self, other: Self) -> Self {
		self.item_list.extend(other.item_list);
		return self.clone()
	}
	pub fn clear_unused(&mut self) -> bool {
		let mut removed_element = false;
		self.item_list.retain(|item|
			if item.equipped {
				true
			} else {
				removed_element = true;
				false
			}
		);
		return removed_element
	}
	pub fn show_item_selection(&mut self, ui: &mut egui::Ui) -> Option<Item> {
		ui.vertical(|ui| {
			for item in &mut self.item_list {
				item.show(ui);
				ui.add_space(5.0);
			}
		});
		return None
	}

	pub fn small_item_count(&self) -> usize {
		self.item_list
			.clone()
			.into_iter()
			.filter(|item| match item.weight {
				Weight::Small => true,
				_ => false
			})
			.count()
	}

	pub fn item_weight(&self) -> usize {
		self.item_list
			.clone()
			.into_iter()
			.fold(usize::MIN, |acc, item| match item.weight {
				Weight::Small => acc + 0,
				Weight::Normal => acc + 1,
				Weight::Heavy => acc + 2,
			})
	}

	pub fn item_weight_custom(&self, weights: [usize; 3]) -> usize {
		self.item_list
			.clone()
			.into_iter()
			.fold(usize::MIN, |acc, item| match item.weight {
				Weight::Small => acc + weights[0],
				Weight::Normal => acc + weights[1],
				Weight::Heavy => acc + weights[2],
			})
	}

	pub fn show_items(&mut self, ui: &mut egui::Ui, weight: Weight) {
		for item in &mut self.item_list {
			if item.weight != weight {
				continue;
			}
			item.show(ui);
			ui.add_space(5.0);
		}
	}
	pub fn show_items_selectable(&mut self, ui: &mut egui::Ui, weight: Weight) -> Option<Item> {
		let mut selected_item: Option<Item> = None;
		for item in &mut self.item_list {
			if item.weight != weight {
				continue;
			}
			if item.show_selectable(ui) {
				let mut temp_item = item.clone();
				temp_item.equipped = true;
				selected_item = Some(temp_item);
			}
			ui.add_space(5.0);
		}
		return selected_item
	}


}
pub fn small_requisition_items() -> ItemList {
	ItemList { item_list: vec![
		Item {
			name: String::from("Knife"),
			weight: Weight::Small,
			..Default::default() }
		,	
		Item {
			name: String::from("Lore Book"),
			weight: Weight::Small,
			effect: Some(String::from("Choose a specific subject. Gives you +2 to knowledge checks on that subject")),
			..Default::default() }
		,
		Item {
			name: String::from("Rope"),
			weight: Weight::Small,
			effect: Some(String::from("Sturdy. Plenty long")),
			..Default::default() }
		,
		Item {
			name: String::from("Climbing Gear"),
			weight: Weight::Small,
			effect: Some(String::from("If you need to climb something you shouldn't. Requires rope")),
			..Default::default() }
		,
		Item {
			name: String::from("Snare Trap"),
			weight: Weight::Small,
			effect: Some(String::from("Immobilises. Full turn setup")),
			..Default::default() }
		,
		Item {
			name: String::from("Lantern"),
			weight: Weight::Small,
			effect: Some(String::from("Creates a dimmable light or spotlight")),
			..Default::default() }
	]}
}
pub fn normal_requisition_items() -> ItemList {
	ItemList { item_list: vec![
		Item {
			name: String::from("Sword"),
			effect: Some(String::from("Finesse: +1 Skill Dice")),
			..Default::default() }
		,
		Item {
			name: String::from("Battle Axe"),
			effect: Some(String::from("(1 SD) Power Attack Rend: +1 Damage")),
			..Default::default() }
		,
		Item {
			name: String::from("Spear"),
			effect: Some(String::from("Reach")),
			..Default::default() }
		,
		Item {
			name: String::from("Recurve Bow"),
			effect: Some(String::from("Long Range")),
			..Default::default() }
		,
		Item {
			name: String::from("Light Armour"),
			effect: Some(String::from("5+ Armour Save")),
			..Default::default() }
		,
		Item {
			name: String::from("Shield"),
			effect: Some(String::from("3+ Cover Save after Block")),
			..Default::default() }
		,
		Item {
			name: String::from("Pack/Satchel"),
			effect: Some(String::from("Storage for 6 Small Items\nSlow to access")),
			..Default::default() }
		,
		Item {
			name: String::from("Belt of pockets"),
			effect: Some(String::from("Storage for 4 Small Items\nCannot be quickly dropped")),
			..Default::default() }
	]}
}
pub fn heavy_requisition_items() -> ItemList {
	ItemList { item_list: vec![
		Item {
			name: String::from("Pole Axe"),
			weight: Weight::Heavy,
			effect: Some(String::from("Reach\n(1 SD) Power Attack Rend: +1 Damage")),
			..Default::default() }
		,
		Item {
			name: String::from("Great Sword"),
			weight: Weight::Heavy,
			effect: Some(String::from("Finesse: +1 Skill Dice\n(1 SD) Power Attack Cleave: Hit extra targets\n(1 SD) Lunge: Treat attacks as Reach")),
			..Default::default() }
		, 
		Item {
			name: String::from("Arbalest Crossbow"),
			weight: Weight::Heavy,
			effect: Some(String::from("Medium Range\nArmour Piercing\nReload: 2 Action Die")),
			..Default::default() }
		,
		Item {
			name: String::from("Armour"),
			weight: Weight::Heavy,
			effect: Some(String::from("+3 Armour Save")),
			..Default::default() }
		,
		Item {
			name: String::from("Tower Shield"),
			weight: Weight::Heavy,
			effect: Some(String::from("+1 Cover Save after Block")),
			..Default::default() }
	]}
}

pub fn all_requisition_items() -> ItemList {
	let mut items = small_requisition_items();
	items.add_item_list(normal_requisition_items());
	items.add_item_list(heavy_requisition_items());
	return items
}