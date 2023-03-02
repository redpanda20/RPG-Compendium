use super::*;

pub fn get_fae_spells(ctx: &egui::Context) -> Vec<Spell> {
	let neg_one = icon::Icon::from_svg_responsive(defines::NEG_ONE.to_vec(), ctx);
	let neg_two = icon::Icon::from_svg_responsive(defines::NEG_TWO.to_vec(), ctx);
	let pixie = icon::Icon::from_svg_constant(defines::PIXIE.to_vec(), ctx);
	let sylviel = icon::Icon::from_svg_constant(defines::SYLVIEL.to_vec(), ctx);
	let forge_s = icon::Icon::from_svg_constant(defines::FORGE_SPRITE.to_vec(), ctx);

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

	Spell{
		spell_type: SpellType::Fae(FaePatron::Pixie),
		tags: vec![
			SpellTags::SelfTarget,
		],
		name: String::from("Determination"),
		symbols: vec![
			neg_one.get(ctx),
			pixie.get(ctx)
		],
		description: String::from("You are not interrupted by the first point of damage each round."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Fae(FaePatron::Pixie),
		tags: vec![
			SpellTags::SelfTarget,
		],
		name: String::from("Regeneration"),
		symbols: vec![
			neg_two.get(ctx),
			pixie.get(ctx),
		],
		description: String::from("All damage regenerates as if it was stun damage: Heal one damage at the end of each round that you were not damaged in."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Fae(FaePatron::Pixie),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Fae Luck"),
		symbols: vec![
			neg_two.get(ctx),
			pixie.get(ctx),
		],
		description: String::from("Once per round as part of a action, you may declare the position and existence as a mundane object which reasonably exists in the situation. The GM may only deny this if the object is out of place. \"I flip the table that is right in front of me.\""),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Fae(FaePatron::Sylviel),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 },
		],
		name: String::from("Illuminating Prayer"),
		symbols: vec![
			neg_one.get(ctx),
			sylviel.get(ctx)
		],
		description: String::from("You may cause objects or creatures to glow. At the start of the round as a free action to can cause a creature or object you can see in medium range to glow. It can no longer benefit from hiding or invisibility and all attacks against it gain +1 accuracy."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Fae(FaePatron::Sylviel),
		tags: vec![
			SpellTags::SelfTarget,
		],
		name: String::from("Prayer of Silence"),
		symbols: vec![
			neg_two.get(ctx),
			sylviel.get(ctx),
		],
		description: String::from("You are entirely silent and cannot make sound of any kind."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Fae(FaePatron::Sylviel),
		tags: vec![],
		name: String::from("Prayer to the Ancestors"),
		symbols: vec![
			neg_two.get(ctx),
			sylviel.get(ctx),
		],
		description: String::from("A warrior spirit defends you. Once per turn it makes a action with a timing of 9 and with a skill of 1 to spend. It can only deal stun damage. When the blessing is cast choose a loadout:\n	- Shield and Sword\n	- Longbow\n	- Pole-axe\nThe spirit is always within close range of you. The blessing ends if it takes any damage. Cannot be cast as a aura."),
		flavour_text: None},
// -------
	Spell{
		spell_type: SpellType::Fae(FaePatron::ForgeSprite),
		tags: vec![
			SpellTags::SelfTarget,
			SpellTags::Attack,
		],
		name: String::from("Flame Breath"),
		symbols: vec![
			neg_one.get(ctx),
			forge_s.get(ctx)
		],
		description: String::from("You have a standard ranged weapon of fire bolts shot from your mouth."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Fae(FaePatron::ForgeSprite),
		tags: vec![
			SpellTags::SelfTarget,
			SpellTags::Attack,
			SpellTags::AreaOfEffect { range_bands: 0..1 }
		],
		name: String::from("Flame Wreath"),
		symbols: vec![
			neg_two.get(ctx),
			forge_s.get(ctx),
		],
		description: String::from("At the end of each round you release a flame pulse that attacks each target you choose in close range."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Fae(FaePatron::ForgeSprite),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 },
		],
		name: String::from("Forge Heart Golem"),
		symbols: vec![
			neg_two.get(ctx),
			forge_s.get(ctx),
		],
		description: String::from("Target medium pile of solid objects become take the heart of a humanoid servant. It obeys orders but is prone to misinterpreting ambiguous ones. Durability and abilities are derived from the object e.g.\n- Rocks. Slow, strong, durable\n- Bones. Average, dexterous\n- Weapons. Dangerous, awkward"),
		flavour_text: None},
	]
}