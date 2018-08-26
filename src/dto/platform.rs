use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
