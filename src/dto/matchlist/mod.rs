mod match_reference;

pub use self::match_reference::MatchReference;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matchlist {
	#[serde(default)]
	pub matches: Vec<MatchReference>,

	#[serde(default)]
	pub total_games: i32,

	#[serde(default)]
	pub start_index: i32,

	#[serde(default)]
	pub end_index: i32,
}
