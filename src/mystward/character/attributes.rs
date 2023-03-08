// use super::*;
use super::spells;

#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Attribute {
	Unused,
	Athletics,
	Melee,
	Marksmanship,
	Stealth,
	FirstAid,
	Diplomacy(Race),
	Magic(spells::SpellType),
	Lore(Lore),
	Blacksmith,
	Explosives,
	Engineering,
	Survivalist,
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
                spells::SpellType::None => "",
                spells::SpellType::Arcane(_) => "Arcane Magic",
                spells::SpellType::Fae(patron) => match patron {
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
#[derive(PartialEq, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Race {
	Cerudant,
	InkurtAnami,
	Rova
}
#[derive(PartialEq, Clone)]
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
pub fn default() -> Vec<(Attribute, u8)> {
    return vec![
        (Attribute::Unused, 6),
        (Attribute::Athletics, 1),
        (Attribute::Melee, 0),
        (Attribute::Marksmanship, 0),
        (Attribute::Stealth, 0),
        (Attribute::FirstAid, 0),	
    ];
}