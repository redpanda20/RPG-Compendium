use super::attributes::Lore;
use super::traits;

#[derive(PartialEq, Clone, Copy)]
pub enum Advance {
	Other,
	TrainMagicFaeSylviel,
	TrainWildMagic,
	TrainMartial(Option<MartialTrait>),
	Study(Option<Lore>),
	Tame(Option<Beast>)
}
#[derive(PartialEq, Clone, Copy)]
pub enum MartialTrait {
	DanceOfBlades,
	DanceOfArrows,
	DanceOfBlood
}
#[derive(PartialEq, Clone, Copy)]
pub enum Beast {
	StarPhoenix,
	ShadowPuma,
	Kirin
}
impl Advance {
	pub fn all_advances() -> Vec<Advance>{
		return vec![
			Advance::Other,
			Advance::TrainWildMagic,
			Advance::TrainMagicFaeSylviel,
			Advance::TrainMartial(None),
			Advance::Study(None),
			Advance::Tame(None)
		]
	}
	pub fn to_string(&self) -> String {
		return match &self {
			Advance::Other => "Other",
			Advance::TrainMagicFaeSylviel => "Fae magic (Sylviel)",
			Advance::TrainWildMagic => "Wild Magic",
			Advance::TrainMartial(martial) => {
				if let Some(martial) = martial {
					match martial {
						MartialTrait::DanceOfBlades => "Train: Dance of Blades",
						MartialTrait::DanceOfArrows => "Train: Dance Of Arrows",
						MartialTrait::DanceOfBlood => "Train: Dance Of Blood",
					}
				} else {
					"No martial trait selected"
				}
			}
            Advance::Study(lore) => {
				if let Some(lore) = lore {
					match lore {
						Lore::Fables => "Study lore (Fables)",
						Lore::Beasts => "Study lore (Beasts)",
						Lore::Plants => "Study lore (Plants)",
						Lore::Commerce => "Study lore (Commerce)",
						Lore::Art => "Study lore (Art)",
						Lore::Architecture => "Study lore (Architecture)",
						Lore::Geology => "Study lore (Geology)",
						Lore::History => "Study lore (History)",
					}
				} else {
					"No lore selected"
				}
            },
			Advance::Tame(beast) => {
				if let Some(beast) = beast {
					match beast {
						Beast::StarPhoenix => "Tame (Star Phoenix)",
						Beast::ShadowPuma => "Tame (Shadow Puma)",
						Beast::Kirin => "Tame (Kirin)",
					}
				} else{
					"No beast selected"
				}
			}
		}.to_string()
	}

	pub fn show_preview(&self, ui: &mut egui::Ui, size: egui::Vec2) -> egui::Response {
		let mut option_response : Option<egui::Response> = None;
		ui.vertical(|ui| {

			let (rect, response) = ui.allocate_at_least(size, egui::Sense::click());
			ui.allocate_ui_at_rect(rect, |ui| {

				let style = ui.style().interact(&response);
				ui.painter().rect(
					rect,
					style.rounding,
					style.bg_fill,
					style.bg_stroke);
		
				match self.clone() {
					Advance::Other => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Other").size(16.0));
						ui.label("A background action which did not grant any new abilities");
					}),
					Advance::TrainMagicFaeSylviel => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Train Fae Magic (Sylviel)").size(16.0));
						ui.label("Unlocks a Fae Magic. Sylviel requires you to partake in arduous rituals and practice but cares little for the morality of her followers");
					}),
					Advance::TrainWildMagic => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Train Wild Magic").size(16.0));
						ui.label("Unlocks wild magic. Wild magic is a fast but unreliable form of casting");
					}),
					Advance::TrainMartial(_) => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Train Martial Skill").size(16.0));
						ui.label("Unlocks a martial skill. Martial skills are non-magical abilities");
					}),
					Advance::Study(_) => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Study Lore").size(16.0));
						ui.label("Unlocks a new Lore. Lore is used for knowledge of something unusual in the world");
					}),
					Advance::Tame(_) => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Tame Beast").size(16.0));
						ui.label("Tame a beast. It becomes your companion");
					}),
					// _ => ui.vertical_centered(|ui| {
					// 	ui.label("Not yet implemented");
					// })
				}
			});
			option_response = Some(response);
		});
		return option_response.unwrap();
	}
	
	pub fn show(&mut self, ui: &mut egui::Ui, size: egui::Vec2) -> egui::Response {
		let mut option_response : Option<egui::Response> = None;
		ui.vertical(|ui| {

			let (rect, response) = ui.allocate_at_least(size, egui::Sense::click());
			ui.allocate_ui_at_rect(rect, |ui| {
				
				let style = ui.style().interact(&response);
				ui.painter().rect(
					rect,
					style.rounding,
					style.bg_fill,
					style.bg_stroke);

				match self.clone() {
					Advance::Other => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Other").size(16.0));
						ui.label("A background action which did not grant any new abilities");
					}),
					Advance::TrainMagicFaeSylviel => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Train Fae Magic (Sylviel)").size(16.0));
						ui.label("Unlocks a Fae Magic. Sylviel requires you to partake in arduous rituals and practice but cares little for the morality of her followers");
					}),
					Advance::TrainWildMagic => ui.vertical_centered(|ui| {
						ui.label(egui::RichText::new("Train Wild Magic").size(16.0));
						ui.label("Unlocks wild magic. Wild magic is a fast but unreliable form of casting");
					}),
					Advance::TrainMartial(martial_trait) => ui.vertical(|ui| {
						ui.vertical_centered(|ui| {
							ui.label(egui::RichText::new("Train Martial Skill").size(16.0));
						});
						egui::ComboBox::from_id_source("Beast combobox")
							.width(ui.available_width())
							.selected_text(self.to_string())
							.show_ui(ui, |ui| {
								ui.selectable_value(self, Advance::TrainMartial(Some(MartialTrait::DanceOfArrows)), "Train: Dance of Arrows");
								ui.selectable_value(self, Advance::TrainMartial(Some(MartialTrait::DanceOfBlades)), "Train: Dance of Blades");
								ui.selectable_value(self, Advance::TrainMartial(Some(MartialTrait::DanceOfBlood)), "Train: Dance of Blood");
						});
						if let Some(martial_trait) = martial_trait {
							ui.label(
							match martial_trait {
								MartialTrait::DanceOfArrows => traits::get_trait(traits::NamedTrait::MartialDanceOfArrows),
								MartialTrait::DanceOfBlades => traits::get_trait(traits::NamedTrait::MartialDanceOfBlades),
								MartialTrait::DanceOfBlood => traits::get_trait(traits::NamedTrait::MartialDanceOfBlood),
							}.text);
						};
					}),
					Advance::Study(_) => ui.vertical(|ui| {
						ui.vertical_centered(|ui| {
							ui.label(egui::RichText::new("Study Lore").size(16.0));
						});
						egui::ComboBox::from_id_source("Lore combobox")
							.width(ui.available_width())
							.selected_text(self.to_string())
							.show_ui(ui, |ui| {
							ui.selectable_value(self, Advance::Study(Some(Lore::Architecture)), "Study lore (Architecture)");
							ui.selectable_value(self, Advance::Study(Some(Lore::Art)), "Study lore (Art)");
							ui.selectable_value(self, Advance::Study(Some(Lore::Beasts)), "Study lore (Beasts)");
							ui.selectable_value(self, Advance::Study(Some(Lore::Commerce)), "Study lore (Commerce)");
							ui.selectable_value(self, Advance::Study(Some(Lore::Fables)), "Study lore (Fables)");
							ui.selectable_value(self, Advance::Study(Some(Lore::Geology)), "Study lore (Geology)");
							ui.selectable_value(self, Advance::Study(Some(Lore::History)), "Study lore (History)");
							ui.selectable_value(self, Advance::Study(Some(Lore::Plants)), "Study lore (Plants)");
						});
					}),
					Advance::Tame(beast) => ui.vertical(|ui| {
						ui.vertical_centered(|ui| {
							ui.label(egui::RichText::new("Tame Beast").size(16.0));
						});
						egui::ComboBox::from_id_source("Beast combobox")
							.width(ui.available_width())
							.selected_text(self.to_string())
							.show_ui(ui, |ui| {
							ui.selectable_value(self, Advance::Tame(Some(Beast::Kirin)), "Tame Beast (Kirin)");
							ui.selectable_value(self, Advance::Tame(Some(Beast::ShadowPuma)), "Tame Beast (Shadow Puma)");
							ui.selectable_value(self, Advance::Tame(Some(Beast::StarPhoenix)), "Tame Beast (Star Phoenix)");
						});
						if let Some(beast) = beast {
							ui.label(
							match beast {
								Beast::StarPhoenix => traits::get_trait(traits::NamedTrait::TamedPhoenix),
								Beast::ShadowPuma => traits::get_trait(traits::NamedTrait::TamedPhoenix),
								Beast::Kirin => traits::get_trait(traits::NamedTrait::TamedKirin),
							}.text);
						};
					}),
					// _ => ui.vertical_centered(|ui| {
					// 	ui.label("Not yet implemented");
					// })
				}
			});
			option_response = Some(response);
		});
		return option_response.unwrap();
	}
}
