mod team_bans;
mod win;

pub use self::team_bans::TeamBans;
pub use self::win::Win;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamStats {
	#[serde(default)]
	pub bans: Vec<TeamBans>,

	#[serde(default)]
	pub baron_kills: i32,

	#[serde(default)]
	pub dominion_victory_score: i32,

	#[serde(default)]
	pub dragon_kills: i32,

	#[serde(default)]
	pub first_baron: bool,

	#[serde(default)]
	pub first_blood: bool,

	#[serde(default)]
	pub first_dragon: bool,

	#[serde(default)]
	pub first_inhibitor: bool,

	#[serde(default)]
	pub first_rift_herald: bool,

	#[serde(default)]
	pub first_tower: bool,

	#[serde(default)]
	pub inhibitor_kills: i32,

	#[serde(default)]
	pub rift_herald_kills: i32,

	#[serde(default)]
	pub team_id: i32,

	#[serde(default)]
	pub tower_kills: i32,

	#[serde(default)]
	pub vilemaw_kills: i32,

	pub win: Win,
}
