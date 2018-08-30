use serde::{ Deserialize, Deserializer, de };
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum Platform {
	BR1,
	EUN1,
	EUW1,
	JP1,
	KR,
	LA1,
	LA2,
	NA,
	NA1,
	OC1,
	TR1,
	RU,
	PBE1,
}
impl fmt::Display for Platform {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Platform::BR1 => write!(f, "BR1"),
			Platform::EUN1 => write!(f, "EUN1"),
			Platform::EUW1 => write!(f, "EUW1"),
			Platform::JP1 => write!(f, "JP1"),
			Platform::KR => write!(f, "KR"),
			Platform::LA1 => write!(f, "LA1"),
			Platform::LA2 => write!(f, "LA2"),
			Platform::NA => write!(f, "NA"),
			Platform::NA1 => write!(f, "NA1"),
			Platform::OC1 => write!(f, "OC1"),
			Platform::TR1 => write!(f, "TR1"),
			Platform::RU => write!(f, "RU"),
			Platform::PBE1 => write!(f, "PBE1"),
		}
	}
}
impl<'de> Deserialize<'de> for Platform {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor;
		impl<'de> de::Visitor<'de> for Visitor {
			type Value = Platform;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str(
					"one of \"BR1\", \"EUN1\", \"EUW1\", \"JP1\", \"KR\", \"LA1\", \"LA2\", \"NA\", \"NA1\", \"OC1\", \"TR1\", \"RU\", \"PBE1\" (case insensitive)"
				)
			}

			fn visit_str<E: de::Error>(self, value: &str) -> Result<Platform, E> {
				match value {
					"BR1" | "br1" => Ok(Platform::BR1),
					"EUN1" | "eun1" => Ok(Platform::EUN1),
					"EUW1" | "euw1" => Ok(Platform::EUW1),
					"JP1" | "jp1" => Ok(Platform::JP1),
					"KR" | "kr" => Ok(Platform::KR),
					"LA1" | "la1" => Ok(Platform::LA1),
					"LA2" | "la2" => Ok(Platform::LA2),
					"NA" | "na" => Ok(Platform::NA),
					"NA1" | "na1" => Ok(Platform::NA1),
					"OC1" | "oc1" => Ok(Platform::OC1),
					"TR1" | "tr1" => Ok(Platform::TR1),
					"RU" | "ru" => Ok(Platform::RU),
					"PBE1" | "pbe1" => Ok(Platform::PBE1),
					_ => Err(E::custom(format!("unknown Platform value: {}", value))),
				}
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}
