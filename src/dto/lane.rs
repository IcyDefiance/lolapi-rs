#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Lane {
	None,
	Top,
	Middle,
	Bottom,
	Jungle,
}
