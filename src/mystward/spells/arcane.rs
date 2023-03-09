use super::*;
use std::collections::HashSet;

pub fn get_arcane_spells(ctx: &egui::Context) -> Vec<Spell> {
	let ignition = icon::Icon::from_svg_constant( defines::IGNITION.to_vec(), ctx);
	let life = icon::Icon::from_svg_constant( defines::LIFE.to_vec(), ctx);
	let design = icon::Icon::from_svg_constant( defines::DESIGN.to_vec(), ctx);
	let astral = icon::Icon::from_svg_constant(defines::ASTRAL.to_vec(), ctx);
	let force = icon::Icon::from_svg_constant(defines::FORCE.to_vec(), ctx);
	let wisdom = icon::Icon::from_svg_constant(defines::WISDOM.to_vec(), ctx);
	let entropy = icon::Icon::from_svg_constant(defines::ENTROPY.to_vec(), ctx);
	return vec![
	// Spell{
	// 	spell_type: SpellType::Arcane(HashSet::from([
	// 		ArcaneConcept::Ignition,
	// 		ArcaneConcept::Ignition,
	// 		ArcaneConcept::Design])),
	// 	tags: vec![
	// 		SpellTags::Attack,
	// 		SpellTags::SingleTarget{targets: 1},
	// 		SpellTags::ArmourPiercing],
	// 	name: String::from("Fire Lance"),
	// 	symbols: vec![
	// 		ignition.get(ctx),
	// 		ignition.get(ctx),
	// 		design.get(ctx)],
	// 	description: String::from("Ignites target for further AP damage at the end of the next round."),
	// 	flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Ignition,
			ArcaneConcept::Ignition])),
		tags: vec![
			SpellTags::Attack,
			SpellTags::AreaOfEffect { range_bands: 0..1 }],
		name: String::from("Fireball"),
		symbols: vec![
			ignition.get(ctx),
			ignition.get(ctx)],
		description: String::from("Close AOE within medium range. It explodes with fire doing 1 point of damage."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Ignition,
			ArcaneConcept::Life])),
		tags: vec![],
		name: String::from("Revive"),
		symbols: vec![
			ignition.get(ctx),
			life.get(ctx)],
		description: String::from("Bring a target back to life, their wounds are NOT healed as part of this spell."),
		flavour_text: Some(String::from("Heal before reviving"))},
	
	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Ignition,
			ArcaneConcept::Design])),
		tags: vec![],
		name: String::from("Construct"),
		symbols: vec![
			ignition.get(ctx),
			design.get(ctx)],
		description: String::from("Close AOE within medium range. It explodes with fire doing 1 point of damage."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Ignition,
			ArcaneConcept::Astral])),
		tags: vec![
			SpellTags::Attack,
			SpellTags::SingleTarget { targets: 3 }],
		name: String::from("Starfire"),
		symbols: vec![
			ignition.get(ctx),
			astral.get(ctx)],
		description: String::from("Target up to 3 targets within medium	range. A lance of starlight strikes down at them for 1 point of damage."),
		flavour_text: Some(String::from("Attacks from above"))},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Ignition,
			ArcaneConcept::Force])),
		tags: vec![
			SpellTags::Attack,
			SpellTags::AreaOfEffect { range_bands: 0..2 }
		],
		name: String::from("Flamethrower"),
		symbols: vec![
			ignition.get(ctx),
			force.get(ctx)],
		description: String::from("Medium range. Extendable with roll. When extended ignores armour."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Ignition,
			ArcaneConcept::Widsom])),
		tags: vec![],
		name: String::from("Inspiration"),
		symbols: vec![
			ignition.get(ctx),
			wisdom.get(ctx)],
		description: String::from("Ask the GM for a plan for the current situation. The GM will give you a plan, it may not be the best plan."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Ignition,
			ArcaneConcept::Entropy])),
		tags: vec![
			SpellTags::Attack,
			SpellTags::AreaOfEffect { range_bands: 0..3 }
		],
		name: String::from("Darkfire"),
		symbols: vec![
			ignition.get(ctx),
			entropy.get(ctx)],
		description: String::from("Ignite a cold black fire, this fire spreads along water but freezes things rather than burn them."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Life,
			ArcaneConcept::Life])),
		tags: vec![
			SpellTags::AreaOfEffect { range_bands: 0..2 }
		],
		name: String::from("Font of Life"),
		symbols: vec![
			life.get(ctx),
			life.get(ctx)],
		description: String::from("Medium AOE centred on you in which plants grow anywhere they can. Extendable 1AD. When extended heals all creatures within the AOE."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Life,
			ArcaneConcept::Design])),
		tags: vec![
			SpellTags::AreaOfEffect { range_bands: 0..2 }
		],
		name: String::from("Ordered Growth"),
		symbols: vec![
			life.get(ctx),
			design.get(ctx)],
		description: String::from("In a close AOE at medium range, plants grow in a way and shape of your design. Can be used to entangle or create cover for example."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Life,
			ArcaneConcept::Astral])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Astral Form"),
		symbols: vec![
			life.get(ctx),
			astral.get(ctx)],
		description: String::from("Become invisible except leaving a subtle trail of floating sparkles as you move."),
		flavour_text: Some(String::from("Extendable 1AD"))},	

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Life,
			ArcaneConcept::Force])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Heal"),
		symbols: vec![
			life.get(ctx),
			force.get(ctx)],
		description: String::from("Touch a character, they are fully healed of their injuries."),
		flavour_text: None},	

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Life,
			ArcaneConcept::Widsom])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Speech"),
		symbols: vec![
			life.get(ctx),
			wisdom.get(ctx)],
		description: String::from("You may converse with any creature in magical tongue. Do not always expect intelligent conversation."),
		flavour_text: Some(String::from("Extendable 1AD"))},	

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Life,
			ArcaneConcept::Entropy])),
		tags: vec![
			SpellTags::Attack,
			SpellTags::SingleTarget { targets: 1 },
		],
		name: String::from("Word of Death"),
		symbols: vec![
			life.get(ctx),
			entropy.get(ctx)],
		description: String::from("Medium range. Damage someone’s life force directly, ignoring all cover, armour or other defences. Is reversed by Revive."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Design,
			ArcaneConcept::Design])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Shape Dirt"),
		symbols: vec![
			design.get(ctx),
			design.get(ctx)],
		description: String::from("Mold soft ground into a solid object. Close range"),
		flavour_text: Some(String::from("Extendable 1AD"))},	
	
	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Design,
			ArcaneConcept::Astral])),
		tags: vec![],
		name: String::from("Imbue"),
		symbols: vec![
			design.get(ctx),
			astral.get(ctx)],
		description: String::from("Imbue an object with pure magic. It causes a AOE magic blast on a specified trigger."),
		flavour_text: None},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Design,
			ArcaneConcept::Force])),
		tags: vec![
			SpellTags::AreaOfEffect { range_bands: 0..1 }
		],
		name: String::from("Shield"),
		symbols: vec![
			design.get(ctx),
			force.get(ctx)],
		description: String::from("Create dome shield that protects a close AOE around you. It blocks movement and effects in or out of the shield and it can only be shattered by a massive force."),
		flavour_text: Some(String::from("Extendable 1AD"))},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Design,
			ArcaneConcept::Widsom])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Illusion"),
		symbols: vec![
			design.get(ctx),
			wisdom.get(ctx)],
		description: String::from("Medium range, one target. Create a illusion affecting one sense of the target. (sight, sound, etc). -1SD for complexity."),
		flavour_text: Some(String::from("Extendable 1AD"))},

	Spell{
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Design,
			ArcaneConcept::Entropy])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Disintegrate"),
		symbols: vec![
			design.get(ctx),
			entropy.get(ctx)],
		description: String::from("Touch an non magical, non living, object. All of the object within a close AOE disintegrates into a fine powder."),
		flavour_text: None},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Astral,
			ArcaneConcept::Astral])),
		tags: vec![
			SpellTags::AreaOfEffect { range_bands: 0..2 }
		],
		name: String::from("Darkness"),
		symbols: vec![
			astral.get(ctx),
			astral.get(ctx)],
		description: String::from("Create a close AOE of Darkness within medium range zone. You can only see the stars on all sides when within or looking into the zone."),
		flavour_text: None},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Astral,
			ArcaneConcept::Force])),
		tags: vec![
			SpellTags::AreaOfEffect { range_bands: 0..1 }
		],
		name: String::from("Gravity Pull"),
		symbols: vec![
			astral.get(ctx),
			force.get(ctx)],
		description: String::from("Target a Close AOE and a single point, both in medium range. Everything in that AOE is pulled to that point. (Interrupts on timing, moves at round end)."),
		flavour_text: None},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Astral,
			ArcaneConcept::Widsom])),
		tags: vec![],
		name: String::from("Scry"),
		symbols: vec![
			astral.get(ctx),
			wisdom.get(ctx)],
		description: String::from("Observe an orbital view directly above your location, you may choose any level of zoom."),
		flavour_text: Some(String::from("Extendable 1AD"))},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Astral,
			ArcaneConcept::Entropy])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Ignore Gravity"),
		symbols: vec![
			astral.get(ctx),
			entropy.get(ctx)],
		description: String::from("Touch range, target ignores gravity."),
		flavour_text: Some(String::from("Extendable 1AD"))},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Force,
			ArcaneConcept::Force])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Relocate"),
		symbols: vec![
			force.get(ctx),
			force.get(ctx)],
		description: String::from("Touch range, target teleports in a unobstructed straight line out to medium range."),
		flavour_text: None},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Force,
			ArcaneConcept::Widsom])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Imprint"),
		symbols: vec![
			force.get(ctx),
			wisdom.get(ctx)],
		description: String::from("Give a target at medium range an idea. They will not automatically act on this idea but they will believe it was their own idea."),
		flavour_text: None},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Force,
			ArcaneConcept::Entropy])),
		tags: vec![
			SpellTags::AreaOfEffect { range_bands: 1..2 }
		],
		name: String::from("Stasis"),
		symbols: vec![
			force.get(ctx),
			entropy.get(ctx)],
		description: String::from("Target a close AOE within medium range. Everything in the zone stops. Actions are interrupted, external effects do not affect anything in the zone etc."),
		flavour_text: Some(String::from("Extendable -1AD"))},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Widsom,
			ArcaneConcept::Widsom])),
		tags: vec![],
		name: String::from("Narrow Possibility"),
		symbols: vec![
			force.get(ctx),
			entropy.get(ctx)],
		description: String::from("Make three statements about a mystery. Each must be entirely unique and plausible. You may not repeat the spell on the same mystery in any way."),
		flavour_text: Some(String::from("The GM will tell you “The truth is among those” or “None of those are true"))},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Widsom,
			ArcaneConcept::Entropy])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Blank"),
		symbols: vec![
			wisdom.get(ctx),
			entropy.get(ctx)],
		description: String::from("Medium range. Target looses all memory for the spell duration or a specific memory forever."),
		flavour_text: Some(String::from("Extendable 1AD"))},

	Spell {
		spell_type: SpellType::Arcane(HashSet::from([
			ArcaneConcept::Entropy,
			ArcaneConcept::Entropy])),
		tags: vec![
			SpellTags::SingleTarget { targets: 1 }
		],
		name: String::from("Enter Void"),
		symbols: vec![
			entropy.get(ctx),
			entropy.get(ctx)],
		description: String::from(""),
		flavour_text: Some(String::from("Target yoursef, You no longer exist. Rip up your character sheet"))},
	]
}