use dto::Platform;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
	#[serde(default)]
	pub account_id: i64,

	#[serde(default)]
	pub current_account_id: i64,

	pub current_platform_id: Platform,

	#[serde(default)]
	pub match_history_uri: String,

	pub platform_id: Platform,

	#[serde(default)]
	pub profile_icon: i32,

	#[serde(default)]
	pub summoner_id: i64,

	#[serde(default)]
	pub summoner_name: String,
}
