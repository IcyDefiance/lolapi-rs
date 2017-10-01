use QueueType;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
	#[serde(default)] pub chest_granted: bool,
	#[serde(default)] pub champion_level: i32,
	#[serde(default)] pub champion_points: i32,
	#[serde(default)] pub champion_id: i64,
	#[serde(default)] pub player_id: i64,
	#[serde(default)] pub champion_points_until_next_level: i64,
	#[serde(default)] pub champion_points_since_last_level: i64,
	#[serde(default)] pub last_play_time: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionList {
	#[serde(default)] pub champions: Vec<Champion>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
	#[serde(default)] pub ranked_play_enabled: bool,
	#[serde(default)] pub bot_enabled: bool,
	#[serde(default)] pub bot_mm_enabled: bool,
	#[serde(default)] pub active: bool,
	#[serde(default)] pub free_to_play: bool,
	#[serde(default)] pub id: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueList {
	#[serde(default)] pub tier: String,
	pub queue: QueueType,
	#[serde(default)] pub name: String,
	#[serde(default)] pub entries: Vec<LeaguePosition>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaguePosition {
	pub queue_type: Option<QueueType>,
	#[serde(default)] pub rank: String,
	#[serde(default)] pub hot_streak: bool,
	pub mini_series: Option<MiniSeries>,
	#[serde(default)] pub wins: i32,
	#[serde(default)] pub veteran: bool,
	#[serde(default)] pub losses: i32,
	#[serde(default)] pub fresh_blood: bool,
	#[serde(default)] pub player_or_team_name: String,
	#[serde(default)] pub inactive: bool,
	#[serde(default)] pub player_or_team_id: String,
	#[serde(default)] pub league_points: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniSeries {
	#[serde(default)] pub wins: i32,
	#[serde(default)] pub losses: i32,
	#[serde(default)] pub target: i32,
	#[serde(default)] pub progress: String,
}
