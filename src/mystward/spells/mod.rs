use crate::resources::images;

mod arcane;
mod fae;

pub struct Spell {
	pub spell_type: SpellType,
	pub tags: Vec<SpellTags>,
	pub name: String,
	pub symbols: Vec<(egui::TextureId, egui::Vec2)>,
	pub description: String,
	pub flavour_text: Option<String>
}
#[derive(PartialEq, Clone)]
pub enum SpellType {
	None,
	Arcane(std::collections::HashSet<ArcaneConcept>),
	Fae(FaePatron)
}
impl Default for SpellType {
    fn default() -> Self {
        SpellType::None
    }
}
#[derive(PartialEq, Clone, Eq, Hash)]
pub enum ArcaneConcept {
	Ignition,
	Life,
	Design,
	Astral,
	Force,
	Widsom,
	Entropy
}
#[derive(PartialEq, Clone, Copy)]
pub enum FaePatron {
	Generic,
	Pixie,
	Sylviel,
	ForgeSprite,
}

#[derive(Clone)]
pub enum SpellTags {
	Attack,
	ArmourPiercing,
	SelfTarget,
	SingleTarget {targets: usize},
	AreaOfEffect {range_bands: std::ops::Range<usize>},
}

impl Spell {

	pub fn show(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
		// let spell = self.clone();
		egui::Frame::menu(&ctx.style())
		.show(ui, |ui| {
			ui.vertical(|ui| {
				// Title bar
				ui.horizontal(|ui| {
					ui.label(egui::RichText::new(self.name.clone()).heading().strong());

					// Align on right
					ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
						ui.spacing_mut().item_spacing = egui::vec2(0.0, 6.0);

						// Rev to keep existing order from struct
						for (id, size) in self.symbols.iter().rev() {
							ui.image(*id, *size);
						};		
					});
				});
				// Spell tags
				ui.horizontal(|ui| {
					for tag in self.tags.iter() {
						let text = match tag {
							SpellTags::Attack => "Attack.",
							SpellTags::ArmourPiercing => "AP.",
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
