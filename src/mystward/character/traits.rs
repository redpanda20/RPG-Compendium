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
			super::ByvineClass::Blessed => traits.push(fae_magic_pixie()),
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
				super::HumanClass::Gambler => traits.extend(vec![Trait::new("Gambler", "You may use Wild Magic for playing games"), wild_magic()]),
			}
		},
		super::RacialArchetype::MoonElf(class) => {
			traits.push(Trait::new("Night vision", "You can clearly see in the dark"));
			match class {
				super::MoonElfClass::Base => (),
				super::MoonElfClass::CerudantRanger => (),
				super::MoonElfClass::StarMage => (),
				super::MoonElfClass::MoonCleric => traits.push(fae_magic_sylviel()),
			}
		},
		super::RacialArchetype::MystFae(class) => {
			traits.push(Trait::new("Ethereal Step", "When applying movement or after a dodge you become Ethereal:\nInvulnerable to damage or effects and can phase through objects."));
			match class {
				super::MystFaeClass::Base => (),
				super::MystFaeClass::WhispersOfWar => traits.push(Trait::new("Whispers of War", "You have a additional Action Die that may only be used to parry or block")),
				super::MystFaeClass::WhispersOfTheUnseen => traits.push(Trait::new("Whispers of the Unseen", "You can ask the GM for random fact about a place, creature, or object you’ve encountered that your character couldn't have known otherwise.")),
				super::MystFaeClass::WhispersOfTheLeylines => traits.push(wild_magic()),
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
		super::RacialArchetype::Wyvren(class) => {
			traits.extend(vec![
				Trait::new("Wings", "You may glide down to lower levels but you do not gain a free normal gear and may not use heavy gear due to your small size."),
				Trait::new("Chaotic Inspiration", "When you create or cast something you may choose to make it more potent at the cost of making it more dangerous to use or more unpredictable.")
			]);
			match class {
				super::WyvrenClass::Base => (),
				super::WyvrenClass::RecklessEngineer => (),
				super::WyvrenClass::ForgeHeart => traits.push(fae_magic_forge_sprite()),
				super::WyvrenClass::WildfireMage => (),
			}
		},
	}
	traits
}
pub fn wild_magic() -> Trait {
	Trait {
		title: String::from("Wild Magic"),
		text: String::new()
	}
}
pub fn fae_magic_pixie() -> Trait {
	Trait {
		title: String::from("Fae magic (Pixie)"),
		text: String::new()
	}
}
pub fn fae_magic_sylviel() -> Trait {
	Trait {
		title: String::from("Fae magic (Sylviel)"),
		text: String::new()
	}
}
pub fn fae_magic_forge_sprite() -> Trait {
	Trait {
		title: String::from("Fae magic (Forge Sprite)"),
		text: String::new()
	}
}
pub fn tamed_phoenix() -> Trait {
	Trait {
		title: String::from("Tamed Star Phoenix"),
		text: String::from("As well as an excellent flight and sight, a Star Phoenix can launch its feathers as medium range pure energy attacks with a timing of 6. After 4 such attacks it will need to rest until the next misison.\nIt refuses to fly in direct sunlight.")
	}
}
pub fn tamed_puma() -> Trait {
	Trait {
		title: String::from("Tamed Shadow Puma"),
		text: String::from("Small for a big cat but still as dangerous as a wolfdog. Shadow Pumas are the undisputed masters of stealth.\nSo good that they will give one close range ally an extra stealth die.\nYou must carry a small item box of fine snacks to appease this picky eater.")
	}
}
pub fn tamed_kirin() -> Trait {
	Trait {
		title: String::from("Tamed Kirin"),
		text: String::from("A goat sized, scaly unicorn chimera. Kirin refuse to do anything useful except lend their power.\nRoll a D6 to randomly select a concept (excluding entropy), the Kirin invokes this concept which may be used by any arcane caster in close range.Once used it then invokes a new random concept at the start of the next round. this continues until it has invoked concepts 4 times in a mission.\nIt will refuse to share its magic while dirty or wet.")
	}
}
pub fn martial_dance_of_arrows() -> Trait {
	Trait {
		title: String::from("Dance of Arrows"),
		text: String::from("When dual wielding melee weapons you may spend 1 SD to attack with both.\nYou may use this effect multiple times per turn.")
	}
}
pub fn martial_dance_of_blades() -> Trait {
	Trait {
		title: String::from("Dance of Blades"),
		text: String::from("On any turn you declare movement. Gain a 5+ evasion save against all attacks.\nDoes not stack with cover saves.")
	}
}
pub fn martial_dance_of_blood() -> Trait {
	Trait {
		title: String::from("Dance of Blood"),
		text: String::from("You gain the following option on any attack:\n	-1 SD: Deal one extra damage to a living unarmoured target. This applies if you would ignore the armour due to accuracy or armour piercing.\n+1 First Aid")
	}
}