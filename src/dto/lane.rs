use serde::{ Deserialize, Deserializer, de };
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Lane {
	None,
	Top,
	Middle,
	Bottom,
	Jungle,
}
impl<'de> Deserialize<'de> for Lane {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor;
		impl<'de> de::Visitor<'de> for Visitor {
			type Value = Lane;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str(
					"one of \"NONE\", \"TOP\", \"MIDDLE\", \"MID\", \"BOTTOM\", \"JUNGLE\""
				)
			}

			fn visit_str<E: de::Error>(self, value: &str) -> Result<Lane, E> {
				match value {
					"NONE" => Ok(Lane::None),
					"TOP" => Ok(Lane::Top),
					"MIDDLE" | "MID" => Ok(Lane::Middle),
					"BOTTOM" => Ok(Lane::Bottom),
					"JUNGLE" => Ok(Lane::Jungle),
					_ => Err(E::custom(format!("unknown Lane value: {}", value))),
				}
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}
