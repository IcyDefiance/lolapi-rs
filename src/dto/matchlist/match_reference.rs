use dto::Platform;
use dto::Season;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchReference {
	#[serde(default)]
	pub champion: i32,

	#[serde(default)]
	pub game_id: i64,

	#[serde(default)]
	pub lane: String,

	pub platform: Platform,

	#[serde(default)]
	pub queue: i32,

	#[serde(default)]
	pub role: String,

	pub season: Season,

	#[serde(with = "ts_milliseconds")]
	pub timestamp: DateTime<Utc>,
}
