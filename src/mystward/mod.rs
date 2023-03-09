pub mod spells;
pub mod character;

pub struct Content {
	pub _all_spells: Vec<spells::Spell>,
	pub requisition_items: character::items::ItemList,
}

pub fn new(ctx: &egui::Context) -> Content {
	Content {
		_all_spells: spells::get_all_spells(ctx),
		requisition_items: character::items::load_requisition_items()
	}
}