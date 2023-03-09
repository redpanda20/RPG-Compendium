#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Trait {
	pub title: String,
	pub text: String,
	// tags: ??
}
impl Trait {
	pub fn new(title: &str, text: &str) -> Trait {
		Trait { title: String::from(title), text: String::from(text) }
	}
}
pub fn from_archetype(archetype: &super::RacialArchetype) -> Vec<Trait> {
	let mut traits = Vec::new();
	match archetype {
		super::RacialArchetype::Undecided => (),
		super::RacialArchetype::Byvine(class) => {
			traits.push(
				Trait::new("Child of the Eternal Story", "Gain inspiration at the start of a adventure and when you risk yourself to save another. You may spend inspiration to ignore one point of damage."));
			match class {
			super::ByvineClass::Base => (),
			super::ByvineClass::Goliath => traits.push(Trait::new("Goliath", "Power attacks don’t cost Skill Dice, heavy gear counts as normal.\nYour horns are a full melee weapon with the option: 0SD: Launch a character into medium range.")),
			super::ByvineClass::RovanKnight => (),
			super::ByvineClass::Blessed => (),
		}},
		super::RacialArchetype::Clank(class) => {
			traits.push(Trait::new("Unnatural Engineer", "Can be healed with Engineering and Magic but not First-aid. You don’t bleed out when down."));
			match class {
			super::ClankClass::Base => (),
			super::ClankClass::Artisnal => traits.push(Trait::new("Artisanal", "You are beautifully crafted. A RP trait causing favourable interactions with those who appreciate art or craftsmanship.")),
			super::ClankClass::Industrial => traits.push(Trait::new("Industrial", "Power attacks don’t cost Skill Dice.\nYou have Natural Armour with a 3+ save")),
			super::ClankClass::Attuned => (),
		}},
		super::RacialArchetype::Human(class) => {
			traits.push(Trait::new("Adventurous Spirit", "Gain Inspiration at the start of a adventure and when you boldly step into the unknown. You may spend Inspiration to gain +1 Action dice at any time."));
			match class {
				super::HumanClass::Base => (),
				super::HumanClass::Pathfinder => traits.push(Trait::new("Pathfinder", "You have a highly trained animal companion, typically a Wolfdog or Hawk. Suitable for both combat and scouting. This animal has plot armour.")),
				super::HumanClass::Rifleman => (),
				super::HumanClass::Gambler => traits.push(Trait::new("Gambler", "You may use Wild Magic for playing games")),
			}
		},
		super::RacialArchetype::MoonElf(class) => {
			traits.push(Trait::new("Night vision", "You can clearly see in the dark"));
			match class {
				super::MoonElfClass::Base => (),
				super::MoonElfClass::CerudantRanger => (),
				super::MoonElfClass::StarMage => (),
				super::MoonElfClass::MoonCleric => (),
			}
		},
		super::RacialArchetype::MystFae(class) => {
			traits.push(Trait::new("Ethereal Step", "When applying movement or after a dodge you become Ethereal:\nInvulnerable to damage or effects and can phase through objects."));
			match class {
				super::MystFaeClass::Base => (),
				super::MystFaeClass::WhispersOfWar => traits.push(Trait::new("Whispers of War", "You have a additional Action Die that may only be used to parry or block")),
				super::MystFaeClass::WhispersOfTheUnseen => traits.push(Trait::new("Whispers of the Unseen", "You can ask the GM for random fact about a place, creature, or object you’ve encountered that your character couldn't have known otherwise.")),
				super::MystFaeClass::WhispersOfTheLeylines => (),
			}
		},
		super::RacialArchetype::Treekin(class) => {
			traits.push(Trait::new("Natural Reliance", "You gain +1 HP which you may loose without incurring a action dice penalty.\nYou may not wear armour though."));
			match class {
				super::TreekinClass::Base => traits.push(Trait::new("Iron Bark", "You have 3+ Natural Armour")),
				super::TreekinClass::IronBark => (),
				super::TreekinClass::NaturalResearcher => (),
				super::TreekinClass::GrowthMage => (),
			}
		},
		super::RacialArchetype::Wyvren(_) => {
			traits.extend(vec![
				Trait::new("Wings", "You may glide down to lower levels but you do not gain a free normal gear and may not use heavy gear due to your small size."),
				Trait::new("Chaotic Inspiration", "When you create or cast something you may choose to make it more potent at the cost of making it more dangerous to use or more unpredictable.")
			]);
		},
	}
	traits
}