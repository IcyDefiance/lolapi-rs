#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamBans {
	#[serde(default)]
	pub champion_id: i32,

	#[serde(default)]
	pub pick_turn: i32,
}
