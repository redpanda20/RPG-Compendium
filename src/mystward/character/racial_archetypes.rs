#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RacialArchetype {
	Undecided,
	Byvine(ByvineClass),
	Clank(ClankClass),
	Human(HumanClass),
	MoonElf(MoonElfClass),
	MystFae(MystFaeClass),
	Treekin(TreekinClass),
	Wyvren(WyvrenClass),
}
impl ToString for RacialArchetype {
    fn to_string(&self) -> String {
		match &self {
			RacialArchetype::Undecided => "",
			RacialArchetype::Byvine(archetype) => match archetype {
				ByvineClass::Base => "",
				ByvineClass::Goliath => "Byvine: Goliath",
				ByvineClass::RovanKnight => "Byvine: Rovan Knight",
				ByvineClass::Blessed => "Byvine: Blessed"
			},
			RacialArchetype::Clank(archetype) => match archetype {
				ClankClass::Base => "",
				ClankClass::Artisnal => "Clank: Artisnal",
				ClankClass::Industrial => "Clank: Industrial",
				ClankClass::Attuned => "Clank: Attuned",
			},
			RacialArchetype::Human(archetype) => match archetype {
				HumanClass::Base => "",
				HumanClass::Pathfinder => "Human: Pathfinder",
				HumanClass::Rifleman => "Human: Rifleman",
				HumanClass::Gambler => "Human: Gambler",
			},
			RacialArchetype::MoonElf(archetype) => match archetype {
				MoonElfClass::Base => "",
				MoonElfClass::CerudantRanger => "Moon Elf: Cerudant Ranger",
				MoonElfClass::StarMage => "Moon Elf: Star mage",
				MoonElfClass::MoonCleric => "Moon Elf: Moon Cleric",
			},
			RacialArchetype::MystFae(archetype) => match archetype {
				MystFaeClass::Base => "",
				MystFaeClass::WhispersOfWar => "Myst Fae: Whispers of War",
				MystFaeClass::WhispersOfTheUnseen => "Myst Fae: Whispers Of The Unseen",
				MystFaeClass::WhispersOfTheLeylines => "Myst Fae: Whispers Of The Ley lines",
			},
			RacialArchetype::Treekin(archetype) => match archetype {
				TreekinClass::Base => "",
				TreekinClass::IronBark => "Treekin: Ironbark",
				TreekinClass::NaturalResearcher => "Treekin: Natural Researcher",
				TreekinClass::GrowthMage => "Treekin: Growth Mage",
			},
			RacialArchetype::Wyvren(archetype) => match archetype {
				WyvrenClass::Base => "",
				WyvrenClass::RecklessEngineer => "Wyvren: Reckless Engineer",
				WyvrenClass::ForgeHeart => "Wyvren: Forge Heart",
				WyvrenClass::WildfireMage => "Wyvren: Wildfire Mage",
			},
    	}.to_string()
	}
}
impl RacialArchetype {
	pub fn get_variants(&self) -> Vec<RacialArchetype> {
		match self {
			RacialArchetype::Undecided => Vec::new(),
			RacialArchetype::Byvine(_) => vec![
				RacialArchetype::Byvine(ByvineClass::Goliath),
				RacialArchetype::Byvine(ByvineClass::RovanKnight),
				RacialArchetype::Byvine(ByvineClass::Blessed),
			],
			RacialArchetype::Clank(_) => vec![
				RacialArchetype::Clank(ClankClass::Artisnal),
				RacialArchetype::Clank(ClankClass::Industrial),
				RacialArchetype::Clank(ClankClass::Attuned)
			],
			RacialArchetype::Human(_) => vec![
				RacialArchetype::Human(HumanClass::Gambler),
				RacialArchetype::Human(HumanClass::Pathfinder),
				RacialArchetype::Human(HumanClass::Rifleman),
			],
			RacialArchetype::MoonElf(_) => vec![
				RacialArchetype::MoonElf(MoonElfClass::CerudantRanger),
				RacialArchetype::MoonElf(MoonElfClass::StarMage),
				RacialArchetype::MoonElf(MoonElfClass::MoonCleric)
			],
			RacialArchetype::MystFae(_) => vec![
				RacialArchetype::MystFae(MystFaeClass::WhispersOfTheLeylines),
				RacialArchetype::MystFae(MystFaeClass::WhispersOfTheUnseen),
				RacialArchetype::MystFae(MystFaeClass::WhispersOfWar)
			],
			RacialArchetype::Treekin(_) => vec![
				RacialArchetype::Treekin(TreekinClass::IronBark),
				RacialArchetype::Treekin(TreekinClass::NaturalResearcher),
				RacialArchetype::Treekin(TreekinClass::GrowthMage),
			],
			RacialArchetype::Wyvren(_) => vec![
				RacialArchetype::Wyvren(WyvrenClass::ForgeHeart),
				RacialArchetype::Wyvren(WyvrenClass::RecklessEngineer),
				RacialArchetype::Wyvren(WyvrenClass::WildfireMage)
			],

		}
	}
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ByvineClass {
	Base,
	Goliath,
	RovanKnight,
	Blessed
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClankClass {
	Base,
	Artisnal,
	Industrial,
	Attuned
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum HumanClass {
	Base,
	Pathfinder,
	Rifleman,
	Gambler
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MoonElfClass {
	Base,
	CerudantRanger,
	StarMage,
	MoonCleric
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MystFaeClass {
	Base,
	WhispersOfWar,
	WhispersOfTheUnseen,
	WhispersOfTheLeylines
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TreekinClass {
	Base,
	IronBark,
	NaturalResearcher,
	GrowthMage
}
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WyvrenClass {
	Base,
	RecklessEngineer,
	ForgeHeart,
	WildfireMage
}