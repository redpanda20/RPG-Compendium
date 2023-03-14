use super::spells;

#[derive(Eq, PartialEq, Clone, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Attribute {
	Unused,
	Athletics,
	Melee,
	Marksmanship,
	Stealth,
	FirstAid,
	Diplomacy(Race),
	Magic(MagicAttribute),
	Lore(Lore),
	Blacksmith,
	Explosives,
	Engineering,
	Survivalist,
}
#[derive(Eq, PartialEq, Clone, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Race {
	Cerudant,
	InkurtAnami,
	Rova
}
#[derive(Eq, PartialEq, Clone, Copy, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Lore {
	Fables,
	Beasts,
	Plants,
	Commerce,
	Art,
	Architecture,
	Geology,
	History
}
#[derive(Eq, PartialEq, Clone, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MagicAttribute {
    Wild,
    Arcane(ArcaneAttribute),
    Fae(spells::FaePatron),
}
#[derive(Eq, PartialEq, Clone, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ArcaneAttribute {
    Practical,
    Theory
}
impl ToString for Attribute {
    fn to_string(&self) -> String {
        match self {
			Attribute::Unused => "Unused",
            Attribute::Athletics => "Athletics",
            Attribute::Melee => "Melee",
            Attribute::Marksmanship => "Marksmanship",
            Attribute::Stealth => "Stealth",
            Attribute::FirstAid => "First Aid",
            Attribute::Diplomacy(race) => match race {
                Race::Cerudant => "Diplomacy (The Cerudant Council)",
                Race::InkurtAnami => "Diplomacy (Inkurt Anami)",
                Race::Rova => "Diplomacy (The Kingdom of Rova)",
            },
            Attribute::Magic(magic_type) => match magic_type {
                MagicAttribute::Wild => "Wild Magic",
                MagicAttribute::Arcane(arcane) => match arcane {
                    ArcaneAttribute::Practical => "Arcane: Practical",
                    ArcaneAttribute::Theory => "Arcane: Theory",
                },
                MagicAttribute::Fae(patron) => match patron {
                    spells::FaePatron::Generic => "Fae Magic",
                    spells::FaePatron::Pixie => "Fae Magic (Pixie)",
                    spells::FaePatron::Sylviel => "Fae Magic (Sylviel)",
                    spells::FaePatron::ForgeSprite => "Fae Magic (Forge Sprite)",
                },
            },
            Attribute::Lore(lore) => match lore {
                Lore::Fables => "Lore (Fables)",
                Lore::Beasts => "Lore (Beasts)",
                Lore::Plants => "Lore (Plants)",
                Lore::Commerce => "Lore (Commerce)",
                Lore::Art => "Lore (Art)",
                Lore::Architecture => "Lore (Architecture)",
                Lore::Geology => "Lore (Geology)",
                Lore::History => "Lore (History)",
            },
            Attribute::Blacksmith => "Blacksmithing",
            Attribute::Explosives => "Explosives",
            Attribute::Engineering => "Engineering",
            Attribute::Survivalist => "Survivalist",
        }.to_owned()
    }
}
pub fn default() -> [(Attribute, u8); 6] {
    return [
        (Attribute::Unused, 6),
        (Attribute::Athletics, 1),
        (Attribute::Melee, 0),
        (Attribute::Marksmanship, 0),
        (Attribute::Stealth, 0),
        (Attribute::FirstAid, 0),
    ]
}
pub fn from_archetype(archetype: &super::RacialArchetype) -> Vec<(Attribute, u8)> {
    let mut attributes = Vec::new();
    match &archetype {
        super::RacialArchetype::Undecided => (),

        super::RacialArchetype::Byvine(class) => {
            attributes.extend(vec![(Attribute::Lore(Lore::Fables), 1)]);
            match class {
                super::ByvineClass::Base => (),
                super::ByvineClass::Goliath => (),
                super::ByvineClass::RovanKnight => attributes.extend(vec![(Attribute::Athletics, 1), (Attribute::Diplomacy(Race::Rova), 1)]),
                super::ByvineClass::Blessed => attributes.extend(vec![(Attribute::Unused, 1), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Theory)), 0), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Practical)), 0)]),
        }},
        super::RacialArchetype::Clank(class) => {
            attributes.extend(vec![(Attribute::Engineering, 1)]); 
            match class {
                super::ClankClass::Base => (),
                super::ClankClass::Artisnal => attributes.extend(vec![(Attribute::Diplomacy(Race::InkurtAnami), 1), (Attribute::Lore(Lore::Commerce), 1), (Attribute::Lore(Lore::Art), 1)]),
                super::ClankClass::Industrial => (),
        
                super::ClankClass::Attuned => attributes.extend(vec![(Attribute::Unused, 1), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Theory)), 0), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Practical)), 0)]),
        }},
        super::RacialArchetype::Human(class) => {
            match class {
                super::HumanClass::Base => (),
                super::HumanClass::Pathfinder => attributes.extend(vec![(Attribute::Survivalist, 1)]),
                super::HumanClass::Rifleman => attributes.extend(vec![(Attribute::Marksmanship, 1), (Attribute::Explosives, 1)]),
                super::HumanClass::Gambler => attributes.extend(vec![(Attribute::Magic(MagicAttribute::Wild), 1)]),
        }},
        super::RacialArchetype::MoonElf(class) => {
            attributes.extend(vec![(Attribute::Survivalist, 1), (Attribute::Unused, 1)]); 
            match class {
                super::MoonElfClass::Base => (),
                super::MoonElfClass::CerudantRanger => attributes.extend(vec![(Attribute::Diplomacy(Race::Cerudant), 1), (Attribute::Unused, 1)]),
                super::MoonElfClass::StarMage => attributes.extend(vec![(Attribute::Unused, 1), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Theory)), 0), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Practical)), 0)]),
                super::MoonElfClass::MoonCleric => attributes.extend(vec![(Attribute::Magic(MagicAttribute::Fae(spells::FaePatron::Sylviel)), 1)]),
        }},
        super::RacialArchetype::MystFae(class) => {
            match class {
                super::MystFaeClass::Base => (),
                super::MystFaeClass::WhispersOfWar => (),
                super::MystFaeClass::WhispersOfTheUnseen => attributes.extend(vec![(Attribute::Unused, 1)]),
                super::MystFaeClass::WhispersOfTheLeylines => attributes.extend(vec![(Attribute::Magic(MagicAttribute::Wild), 1)]),
        }},
        super::RacialArchetype::Treekin(class) => {
            match class {
                super::TreekinClass::Base => (),
                super::TreekinClass::IronBark => attributes.extend(vec![(Attribute::Melee, 1)]),
                super::TreekinClass::NaturalResearcher => attributes.extend(vec![(Attribute::Lore(Lore::Beasts), 1), (Attribute::Lore(Lore::Plants), 1), (Attribute::Unused, 1)]),
                super::TreekinClass::GrowthMage => attributes.extend(vec![(Attribute::Unused, 1), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Theory)), 0), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Practical)), 0)]),
        }},
        super::RacialArchetype::Wyvren(class) => {
            match class {
                super::WyvrenClass::Base => (),
                super::WyvrenClass::RecklessEngineer => attributes.extend(vec![(Attribute::Engineering, 1)]),
                super::WyvrenClass::ForgeHeart => attributes.extend(vec![(Attribute::Magic(MagicAttribute::Fae(spells::FaePatron::ForgeSprite)), 1), (Attribute::Blacksmith, 1)]),
                super::WyvrenClass::WildfireMage => attributes.extend(vec![(Attribute::Unused, 1), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Theory)), 0), (Attribute::Magic(MagicAttribute::Arcane(ArcaneAttribute::Practical)), 0)]),
        }},
    };
    attributes
}