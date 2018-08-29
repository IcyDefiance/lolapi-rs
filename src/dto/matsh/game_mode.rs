#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameMode {
	Classic,
	Odin,
	Aram,
	Tutorial,
	Urf,
	Doombotsteemo,
	Oneforall,
	Ascension,
	Firstblood,
	Kingporo,
	Siege,
	Assassinate,
	Arsr,
	Darkstar,
	Starguardian,
	Project,
	Gamemodex,
}
