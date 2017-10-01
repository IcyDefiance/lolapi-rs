use serde::de;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QueueType {
	RankedSolo5x5,
	RankedFlexSR,
	RankedFlexTT,
}
impl QueueType {
	pub(super) fn to_str(self) -> &'static str {
		match self {
			QueueType::RankedSolo5x5 => "RANKED_SOLO_5x5",
			QueueType::RankedFlexSR => "RANKED_FLEX_SR",
			QueueType::RankedFlexTT => "RANKED_FLEX_TT",
		}
	}
}
impl<'de> de::Deserialize<'de> for QueueType {
	fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<QueueType, D::Error> {
		struct QueueTypeVisitor;
		impl<'de> de::Visitor<'de> for QueueTypeVisitor {
			type Value = QueueType;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("\"RANKED_SOLO_5x5\", \"RANKED_FLEX_SR\", or \"RANKED_FLEX_TT\"")
			}

			fn visit_str<E: de::Error>(self, value: &str) -> Result<QueueType, E> {
				match value {
					"RANKED_SOLO_5x5" => Ok(QueueType::RankedSolo5x5),
					"RANKED_FLEX_SR" => Ok(QueueType::RankedFlexSR),
					"RANKED_FLEX_TT" => Ok(QueueType::RankedFlexTT),
					_ => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
				}
			}
		}

		deserializer.deserialize_str(QueueTypeVisitor)
	}
}
