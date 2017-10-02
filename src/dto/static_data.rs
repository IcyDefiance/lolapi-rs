use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionList {
	#[serde(default)] pub keys: HashMap<String, String>,
	#[serde(default)] pub data: HashMap<String, Champion>,
	#[serde(default)] pub version: String,
	#[serde(default)] pub data_type: String,
	#[serde(default)] pub format: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
	pub info: Option<Info>,
	#[serde(default)] pub enemytyps: Vec<String>,
	pub stats: Option<Stats>,
	#[serde(default)] pub name: String,
	#[serde(default)] pub title: String,
	pub image: Option<Image>,
	#[serde(default)] pub tags: Vec<String>,
	#[serde(default)] pub partype: String,
	#[serde(default)] pub skins: Vec<Skin>,
	pub passive: Option<Passive>,
	#[serde(default)] pub recommended: Vec<Recommended>,
	#[serde(default)] pub allytips: Vec<String>,
	#[serde(default)] pub key: String,
	#[serde(default)] pub lore: String,
	#[serde(default)] pub id: i32,
	#[serde(default)] pub blurb: String,
	#[serde(default)] pub spells: Vec<ChampionSpell>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
	#[serde(default)] pub difficulty: i32,
	#[serde(default)] pub attack: i32,
	#[serde(default)] pub defense: i32,
	#[serde(default)] pub magic: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
	#[serde(default)] pub armorperlevel: f64,
	#[serde(default)] pub hpperlevel: f64,
	#[serde(default)] pub attackdamage: f64,
	#[serde(default)] pub mpperlevel: f64,
	#[serde(default)] pub attackspeedoffset: f64,
	#[serde(default)] pub armor: f64,
	#[serde(default)] pub hp: f64,
	#[serde(default)] pub hpregenperlevel: f64,
	#[serde(default)] pub spellblock: f64,
	#[serde(default)] pub attackrange: f64,
	#[serde(default)] pub movespeed: f64,
	#[serde(default)] pub attackdamageperlevel: f64,
	#[serde(default)] pub mpregenperlevel: f64,
	#[serde(default)] pub mp: f64,
	#[serde(default)] pub spellblockperlevel: f64,
	#[serde(default)] pub crit: f64,
	#[serde(default)] pub mpregen: f64,
	#[serde(default)] pub attackspeedperlevel: f64,
	#[serde(default)] pub hpregen: f64,
	#[serde(default)] pub critperlevel: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
	#[serde(default)] pub full: String,
	#[serde(default)] pub group: String,
	#[serde(default)] pub sprite: String,
	#[serde(default)] pub h: i32,
	#[serde(default)] pub w: i32,
	#[serde(default)] pub y: i32,
	#[serde(default)] pub x: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
	#[serde(default)] pub num: i32,
	#[serde(default)] pub name: String,
	#[serde(default)] pub id: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
	pub image: Image,
	#[serde(default)] pub sanitized_description: String,
	#[serde(default)] pub name: String,
	#[serde(default)] pub description: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommended {
	#[serde(default)] pub map: String,
	#[serde(default)] pub blocks: Vec<Block>,
	#[serde(default)] pub champion: String,
	#[serde(default)] pub title: String,
	#[serde(default)] pub priority: bool,
	#[serde(default)] pub mode: String,
	#[serde(default)] pub blocks_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
	#[serde(default)] pub items: Vec<BlockItem>,
	#[serde(default)] pub rec_math: bool,
	#[serde(default)] pub items_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockItem {
	#[serde(default)] pub count: i32,
	#[serde(default)] pub id: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionSpell {
	#[serde(default)] pub cooldown_burn: String,
	#[serde(default)] pub resource: String,
	pub leveltip: LevelTip,
	#[serde(default)] pub vars: Vec<SpellVars>,
	#[serde(default)] pub cost_type: String,
	pub image: Image,
	#[serde(default)] pub sanitized_description: String,
	#[serde(default)] pub sanitized_tooltip: String,
	#[serde(default)] pub effect: Vec<Vec<f64>>,
	#[serde(default)] pub tooltip: String,
	#[serde(default)] pub maxrank: i32,
	#[serde(default)] pub cost_burn: String,
	#[serde(default)] pub range_burn: String,
	// pub range // TODO: make this field work
	#[serde(default)] pub cooldown: Vec<f64>,
	#[serde(default)] pub cost: Vec<i32>,
	#[serde(default)] pub key: String,
	#[serde(default)] pub description: String,
	#[serde(default)] pub effect_burn: Vec<String>,
	#[serde(default)] pub altimages: Vec<Image>,
	#[serde(default)] pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelTip {
	#[serde(default)] pub effect: Vec<String>,
	#[serde(default)] pub label: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellVars {
	#[serde(default)] pub ranks_with: String,
	#[serde(default)] pub dyn: String,
	#[serde(default)] pub link: String,
	#[serde(default)] pub coeff: Vec<f64>,
	#[serde(default)] pub key: String,
}
