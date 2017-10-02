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
