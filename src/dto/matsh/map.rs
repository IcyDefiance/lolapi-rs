use serde::{ Deserialize, Deserializer, Serialize, Serializer, de };
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Map {
	SummonersRift = 1,
	SummonersRift = 2,
	TheProvingGrounds = 3,
	TwistedTreeline = 4,
	TheCrystalScar = 8,
	TwistedTreeline = 10,
	SummonersRift = 11,
	HowlingAbyss = 12,
	ButchersBridge = 14,
	CosmicRuins = 16,
	ValoranCityPark = 18,
	Substructure43 = 19,
	NexusBlitz = 21,
}
impl Serialize for Map {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_u64(*self as u64)
	}
}
impl<'de> Deserialize<'de> for Map {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor;
		impl<'de> de::Visitor<'de> for Visitor {
			type Value = Map;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("positive integer")
			}

			fn visit_u64<E: de::Error>(self, value: u64) -> Result<Map, E> {
				match value {
					1 => Ok(Map::SummonersRift),
					2 => Ok(Map::SummonersRift),
					3 => Ok(Map::TheProvingGrounds),
					4 => Ok(Map::TwistedTreeline),
					8 => Ok(Map::TheCrystalScar),
					10 => Ok(Map::TwistedTreeline),
					11 => Ok(Map::SummonersRift),
					12 => Ok(Map::HowlingAbyss),
					14 => Ok(Map::ButchersBridge),
					16 => Ok(Map::CosmicRuins),
					18 => Ok(Map::ValoranCityPark),
					19 => Ok(Map::Substructure43),
					21 => Ok(Map::NexusBlitz),
					_ => Err(E::custom(format!("unknown Map value: {}", value))),
				}
			}
		}

		deserializer.deserialize_u64(Visitor)
	}
}
impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Map::SummonersRift => write!(f, "Summoner's Rift"),
			Map::SummonersRift => write!(f, "Summoner's Rift"),
			Map::TheProvingGrounds => write!(f, "The Proving Grounds"),
			Map::TwistedTreeline => write!(f, "Twisted Treeline"),
			Map::TheCrystalScar => write!(f, "The Crystal Scar"),
			Map::TwistedTreeline => write!(f, "Twisted Treeline"),
			Map::SummonersRift => write!(f, "Summoner's Rift"),
			Map::HowlingAbyss => write!(f, "Howling Abyss"),
			Map::ButchersBridge => write!(f, "Butcher's Bridge"),
			Map::CosmicRuins => write!(f, "Cosmic Ruins"),
			Map::ValoranCityPark => write!(f, "Valoran City Park"),
			Map::Substructure43 => write!(f, "Substructure 43"),
			Map::NexusBlitz => write!(f, "Nexus Blitz"),
		}
	}
}
