use QueueType;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
	pub chest_granted: bool,
	pub champion_level: i32,
	pub champion_points: i32,
	pub champion_id: i64,
	pub player_id: i64,
	pub champion_points_until_next_level: i64,
	pub champion_points_since_last_level: i64,
	pub last_play_time: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionList {
	pub champions: Vec<Champion>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
	pub ranked_play_enabled: bool,
	pub bot_enabled: bool,
	pub bot_mm_enabled: bool,
	pub active: bool,
	pub free_to_play: bool,
	pub id: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueList {
	pub tier: String,
	pub queue: QueueType,
	pub name: String,
	pub entries: Vec<LeaguePosition>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaguePosition {
	pub rank: String,
	pub hot_streak: bool,
	pub mini_series: MiniSeries,
	pub wins: i32,
	pub veteran: bool,
	pub losses: i32,
	pub fresh_blood: bool,
	pub player_or_team_name: String,
	pub inactive: bool,
	pub player_or_team_id: String,
	pub league_points: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniSeries {
	pub wins: i32,
	pub losses: i32,
	pub target: i32,
	pub progress: String,
}
