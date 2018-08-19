#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mastery {
	#[serde(default)]
	pub mastery_id: i32,

	#[serde(default)]
	pub rank: i32,
}
