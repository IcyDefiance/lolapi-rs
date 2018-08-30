use serde::{ Deserialize, Deserializer, Serialize, Serializer, de };
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
impl Serialize for Platform {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&self.to_string())
	}
}
impl<'de> Deserialize<'de> for Platform {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor;
		impl<'de> de::Visitor<'de> for Visitor {
			type Value = Platform;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("one of `BR1`, `EUN1`, `EUW1`, `JP1`, `KR`, `LA1`, `LA2`, `NA`, `NA1`, `OC1`, `TR1`, `RU`, `PBE1` (case insensitive)")
			}

			fn visit_str<E: de::Error>(self, value: &str) -> Result<Platform, E> {
				match value.to_uppercase().as_ref() {
					"BR1" => Ok(Platform::BR1),
					"EUN1" => Ok(Platform::EUN1),
					"EUW1" => Ok(Platform::EUW1),
					"JP1" => Ok(Platform::JP1),
					"KR" => Ok(Platform::KR),
					"LA1" => Ok(Platform::LA1),
					"LA2" => Ok(Platform::LA2),
					"NA" => Ok(Platform::NA),
					"NA1" => Ok(Platform::NA1),
					"OC1" => Ok(Platform::OC1),
					"TR1" => Ok(Platform::TR1),
					"RU" => Ok(Platform::RU),
					"PBE1" => Ok(Platform::PBE1),
					_ => Err(E::custom(format!("unknown Platform value: {}", value))),
				}
			}
		}

		deserializer.deserialize_u64(Visitor)
	}
}
