use super::*;

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Item {
	name: String,
	flavour_text: Option<String>,
	weight: Weight,
	attribute_bonus: Vec<(Attribute, u8)>,
	effect: Option<String>
}
#[derive(Clone)]
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
			attribute_bonus: Vec::new(),
			effect: None
		}
    }
}
impl Item {
	pub fn show(&self, ui: &mut egui::Ui) {		
		ui.vertical(|ui| {
			ui.heading(&self.name);
			for (attribute, quantity) in &self.attribute_bonus {
				ui.label(format!("{}: +{}", attribute.to_string(), quantity));
			}
			if let Some(text) = &self.effect {
				ui.label(text);
			}
			if let Some(text) = &self.flavour_text {
				ui.label(egui::RichText::new(text).italics());
			}
		});
	}
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ItemList {
	pub item_list: Vec<Item>
}
impl ItemList {
	pub fn show(&self, ui: &mut egui::Ui) {
		let small_items: Vec<Item> = self.item_list
			.clone()
			.into_iter()
			.filter(|item| match item.weight {
				Weight::Small => true,
				_ => false,
			})
			.collect();
		let normal_items: Vec<Item> = self.item_list
			.clone()
			.into_iter()
			.filter(|item| match item.weight {
				Weight::Normal => true,
				_ => false,
			})
			.collect();
		let heavy_items: Vec<Item> = self.item_list
			.clone()
			.into_iter()
			.filter(|item| match item.weight {
				Weight::Heavy => true,
				_ => false,
			})
			.collect();

		ui.centered_and_justified(|ui| {
			ui.label(egui::RichText::new("Small Items").size(16.0));
		});
		ui.vertical(|ui| {
			for item in small_items {
				ui.horizontal(|ui| {
					ui.label("-");
					item.show(ui);
				});
				ui.add_space(5.0);
			}
		});
		ui.separator();
		ui.vertical(|ui| {
			for item in normal_items {
				item.show(ui);
				ui.add_space(10.0);
			}
		});
		ui.vertical(|ui| {
			for item in heavy_items {
				ui.horizontal(|ui| {
					item.show(ui);
					ui.with_layout(
						egui::Layout::right_to_left(egui::Align::Min), 
						|ui| { ui.label("Heavy") }
					);
				});
				ui.add_space(10.0);
			}
		});

	}
}
pub fn load_requisition_items() -> ItemList {
	ItemList { item_list: vec![

		Item {
			name: String::from("Knife"),
			weight: Weight::Small,
			..Default::default() }
		,	
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
			name: String::from("Recurve Bow"),
			effect: Some(String::from("Long Range")),
			..Default::default() }
		,
		Item {
			name: String::from("Arbalest Crossbow"),
			weight: Weight::Heavy,
			effect: Some(String::from("Medium Range\nArmour Piercing\nReload: 2 Action Die")),
			..Default::default() }
		,
		Item {
			name: String::from("Light Armour"),
			effect: Some(String::from("5+ Armour Save")),
			..Default::default() }
		,
		Item {
			name: String::from("Heavy Armour"),
			weight: Weight::Heavy,
			effect: Some(String::from("+3 Armour Save")),
			..Default::default() }
		,
		Item {
			name: String::from("Shield"),
			effect: Some(String::from("3+ Cover Save after Block")),
			..Default::default() }
		,
		Item {
			name: String::from("Tower Shield"),
			weight: Weight::Heavy,
			effect: Some(String::from("+1 Cover Save after Block")),
			..Default::default() }
		,
		Item {
			name: String::from("Pack/Satchel"),
			effect: Some(String::from("Storage for 6 Small Items\nSlow to acess")),
			..Default::default() }
		,
		Item {
			name: String::from("Belt of pockets"),
			effect: Some(String::from("Storage for 4 Small Items\nCannot be quickly dropped")),
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
		,
	]}
}