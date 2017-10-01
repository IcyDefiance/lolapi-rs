#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
	chest_granted: bool,
	champion_level: i32,
	champion_points: i32,
	champion_id: i64,
	player_id: i64,
	champion_points_until_next_level: i64,
	champion_points_since_last_level: i64,
	last_play_time: i64,
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
