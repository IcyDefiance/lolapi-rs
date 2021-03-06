use chrono::{ NaiveDateTime, naive::serde::ts_milliseconds };
use dto::Lane;
use dto::Platform;
use dto::Role;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchReference {
	#[serde(default)]
	pub game_id: i64,

	#[serde(default)]
	pub champion: i32,

	pub lane: Option<Lane>,

	pub platform_id: Platform,

	#[serde(default)]
	pub queue: i32,

	pub role: Option<Role>,

	#[serde(default)]
	pub season: i16,

	#[serde(with = "ts_milliseconds")]
	pub timestamp: NaiveDateTime,
}
