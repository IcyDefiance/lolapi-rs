mod team_bans;

pub use self::team_bans::TeamBans;

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

	pub first_baron: bool,

	pub first_blood: bool,

	pub first_dragon: bool,

	pub first_inhibitor: bool,

	pub first_rift_herald: bool,

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

	#[serde(default)]
	pub win: String,
}
