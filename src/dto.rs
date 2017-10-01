#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
	chest_granted: bool,
	champion_level: u32,
	champion_points: u32,
	champion_id: u64,
	player_id: u64,
	champion_points_until_next_level: u64,
	champion_points_since_last_level: u64,
	last_play_time: u64,
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
	pub id: u64,
}
