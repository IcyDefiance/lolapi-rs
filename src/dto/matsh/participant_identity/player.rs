#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
	#[serde(default)]
	pub account_id: i64,

	#[serde(default)]
	pub current_account_id: i64,

	#[serde(default)]
	pub current_platform_id: String,

	#[serde(default)]
	pub match_history_uri: String,

	#[serde(default)]
	pub platform_id: String,

	#[serde(default)]
	pub profile_icon: i32,

	#[serde(default)]
	pub summoner_id: i64,

	#[serde(default)]
	pub summoner_name: String,
}
