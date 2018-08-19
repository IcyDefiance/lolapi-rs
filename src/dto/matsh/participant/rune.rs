#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
	#[serde(default)]
	pub rune_id: i32,

	#[serde(default)]
	pub rank: i32,
}
