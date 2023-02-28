use super::*;

pub fn get_fae_spells(ctx: &egui::Context) -> Vec<Spell> {
	let neg_one = images::StaticSvg::new(String::from("neg-one"), images::NEG_ONE.to_vec());
	let neg_two = images::StaticSvg::new(String::from("neg-two"), images::NEG_TWO.to_vec());

	return vec![
	Spell{
		spell_type: SpellType::Fae(FaePatron::Generic),
		tags: vec![
			SpellTags::SingleTarget {targets: 1},
		],
		name: String::from("Blessing of Protection"),
		symbols: vec![
			neg_one.get(ctx)
		],
		description: String::from("Target Armour or Shield grants one step better saves."),
		flavour_text: None},	

	Spell{
		spell_type: SpellType::Fae(FaePatron::Generic),
		tags: vec![
			SpellTags::SingleTarget {targets: 1},
		],
		name: String::from("Bless Blade"),
		symbols: vec![
			neg_one.get(ctx)
		],
		description: String::from("Target Weapon grants a +1 timing bonus to related rolls."),
		flavour_text: None},	

	Spell{
		spell_type: SpellType::Fae(FaePatron::Generic),
		tags: vec![
			SpellTags::SelfTarget,
		],
		name: String::from("Blessing of the Pilgrim"),
		symbols: vec![
			neg_one.get(ctx)
		],
		description: String::from("No creature will assume you are hostile unless you take hostile action."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Fae(FaePatron::Generic),
		tags: vec![
			SpellTags::SelfTarget,
		],
		name: String::from("Blessing of Grace"),
		symbols: vec![
			neg_two.get(ctx)
		],
		description: String::from("You gain +1 Action Dice, provided you can still use one of your base AD."),
		flavour_text: None},	
	]
}