use serde::{ Deserialize, Deserializer, Serialize, Serializer, de };
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Season {
	Preseason3 = 0,
	Season3 = 1,
	Preseason2014 = 2,
	Season2014 = 3,
	Preseason2015 = 4,
	Season2015 = 5,
	Preseason2016 = 6,
	Season2016 = 7,
	Preseason2017 = 8,
	Season2017 = 9,
	Preseason2018 = 10,
	Season2018 = 11,
}
impl Serialize for Season {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_u64(*self as u64)
	}
}
impl<'de> Deserialize<'de> for Season {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor;
		impl<'de> de::Visitor<'de> for Visitor {
			type Value = Season;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("positive integer")
			}

			fn visit_u64<E: de::Error>(self, value: u64) -> Result<Season, E> {
				match value {
					0 => Ok(Season::Preseason3),
					1 => Ok(Season::Season3),
					2 => Ok(Season::Preseason2014),
					3 => Ok(Season::Season2014),
					4 => Ok(Season::Preseason2015),
					5 => Ok(Season::Season2015),
					6 => Ok(Season::Preseason2016),
					7 => Ok(Season::Season2016),
					8 => Ok(Season::Preseason2017),
					9 => Ok(Season::Season2017),
					10 => Ok(Season::Preseason2018),
					11 => Ok(Season::Season2018),
					_ => Err(E::custom(format!("unknown Season value: {}", value))),
				}
			}
		}

		deserializer.deserialize_u64(Visitor)
	}
}
impl fmt::Display for Season {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Season::Preseason3 => write!(f, "PRESEASON 3"),
			Season::Season3 => write!(f, "SEASON 3"),
			Season::Preseason2014 => write!(f, "PRESEASON 2014"),
			Season::Season2014 => write!(f, "SEASON 2014"),
			Season::Preseason2015 => write!(f, "PRESEASON 2015"),
			Season::Season2015 => write!(f, "SEASON 2015"),
			Season::Preseason2016 => write!(f, "PRESEASON 2016"),
			Season::Season2016 => write!(f, "SEASON 2016"),
			Season::Preseason2017 => write!(f, "PRESEASON 2017"),
			Season::Season2017 => write!(f, "SEASON 2017"),
			Season::Preseason2018 => write!(f, "PRESEASON 2018"),
			Season::Season2018 => write!(f, "SEASON 2018"),
		}
	}
}
