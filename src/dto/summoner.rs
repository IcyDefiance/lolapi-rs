#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
	#[serde(default)]
	pub id: i64,

	#[serde(default)]
	pub account_id: i64,

	#[serde(default)]
	pub name: String,

	#[serde(default)]
	pub profile_icon_id: i32,

	#[serde(default)]
	pub revision_date: i64,

	#[serde(default)]
	pub summoner_level: i64,
}
