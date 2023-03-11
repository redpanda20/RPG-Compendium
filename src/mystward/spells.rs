use crate::resources::{icon, defines};

mod arcane;
mod fae;

#[derive(Clone)]
pub struct Spell {
	pub spell_type: SpellType,
	pub tags: Vec<SpellTags>,
	pub name: String,
	pub symbols: Vec<(egui::TextureId, egui::Vec2)>,
	pub description: String,
	pub flavour_text: Option<String>
}
#[derive(Eq, PartialEq, Clone, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CasterType {
	Arcane(ArcaneConcept),
	Fae(FaePatron),
	Wild
}
#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SpellType {
	None,
	Arcane(std::collections::HashSet<ArcaneConcept>),
	Fae(FaePatron),
	Wild
}
impl Default for SpellType {
    fn default() -> Self {
        SpellType::None
    }
}
#[derive(PartialEq, Clone, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ArcaneConcept {
	Ignition,
	Life,
	Design,
	Astral,
	Force,
	Widsom,
	Entropy
}
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FaePatron {
	Generic,
	Pixie,
	Sylviel,
	ForgeSprite,
}

#[derive(Clone)]
pub enum SpellTags {
	Attack,
	#[allow(dead_code)]
	ArmourPiercing,
	Aura,
	SelfTarget,
	SingleTarget {targets: usize},
	AreaOfEffect {range_bands: std::ops::Range<usize>},
}

impl Spell {

	pub fn show(&self, ui: &mut egui::Ui, ctx: &egui::Context, width: f32) {
		egui::Frame::menu(&ctx.style())
		.show(ui, |ui| {
			ui.set_width(width);
			
		ui.vertical(|ui| {
			// Title bar
			ui.horizontal(|ui| {
				ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 6.0);
				for (id, size) in self.symbols.iter() {
					ui.image(*id, *size);
				};
				ui.style_mut().spacing.item_spacing = egui::vec2(6.0, 6.0);
				ui.separator();

				ui.heading(self.name.clone());

			});
			// Spell tags
			ui.horizontal(|ui| {
				for tag in self.tags.iter() {
					let text = match tag {
						SpellTags::Attack => "Attack.",
						SpellTags::ArmourPiercing => "AP.",
						SpellTags::Aura => "Self / Aura.",
						SpellTags::SelfTarget => "Self.",
						SpellTags::SingleTarget { targets: _ } => "Single Target.",
						SpellTags::AreaOfEffect { range_bands: _ } => "Area of Effect."};
					ui.label(
						egui::RichText::new(text)
							.strong()
					);
					ui.add_space(4.0);
				}
			});
			// Body text
			ui.label(self.description.clone());
			if let Some(flavour_text) = self.flavour_text.clone() {
				ui.label(
					egui::RichText::new(flavour_text)
						.weak()
						.italics()
				);
			}
		});
		});
	}
}

pub fn get_all_spells(ctx: &egui::Context) -> Vec<Spell> {
	let mut spells = Vec::new();
	spells.extend(arcane::get_arcane_spells(ctx));
	spells.extend(fae::get_fae_spells(ctx));
	return spells
}
