use QueueType;
use serde::de;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
	#[serde(default)] pub chest_granted: bool,
	#[serde(default)] pub champion_level: i32,
	#[serde(default)] pub champion_points: i32,
	#[serde(default)] pub champion_id: i64,
	#[serde(default)] pub player_id: i64,
	#[serde(default)] pub champion_points_until_next_level: i64,
	#[serde(default)] pub champion_points_since_last_level: i64,
	#[serde(default)] pub last_play_time: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueList {
	#[serde(default)] pub tier: String,
	pub queue: QueueType,
	#[serde(default)] pub name: String,
	#[serde(default)] pub entries: Vec<LeaguePosition>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaguePosition {
	pub queue_type: Option<QueueType>,
	#[serde(default)] pub rank: String,
	#[serde(default)] pub hot_streak: bool,
	pub mini_series: Option<MiniSeries>,
	#[serde(default)] pub wins: i32,
	#[serde(default)] pub veteran: bool,
	#[serde(default)] pub losses: i32,
	#[serde(default)] pub fresh_blood: bool,
	#[serde(default)] pub player_or_team_name: String,
	#[serde(default)] pub inactive: bool,
	#[serde(default)] pub player_or_team_id: String,
	#[serde(default)] pub league_points: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniSeries {
	#[serde(default)] pub wins: i32,
	#[serde(default)] pub losses: i32,
	#[serde(default)] pub target: i32,
	#[serde(default)] pub progress: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionListDynamic {
	#[serde(default)] pub champions: Vec<ChampionDynamic>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionDynamic {
	#[serde(default)] pub ranked_play_enabled: bool,
	#[serde(default)] pub bot_enabled: bool,
	#[serde(default)] pub bot_mm_enabled: bool,
	#[serde(default)] pub active: bool,
	#[serde(default)] pub free_to_play: bool,
	#[serde(default)] pub id: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionListStatic {
	#[serde(default)] pub keys: HashMap<String, String>,
	#[serde(default)] pub data: HashMap<String, ChampionStatic>,
	#[serde(default)] pub version: String,
	#[serde(default)] pub data_type: String,
	#[serde(default)] pub format: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStatic {
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
	pub range: Option<Range>,
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemList {
	#[serde(default)] pub data: HashMap<String, Item>,
	#[serde(default)] pub version: String,
	#[serde(default)] pub tree: Vec<ItemTree>,
	#[serde(default)] pub groups: Vec<Group>,
	#[serde(default)] pub items_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemTree {
	#[serde(default)] pub header: String,
	#[serde(default)] pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
	pub gold: Option<Gold>,
	#[serde(default)] pub plaintext: String,
	#[serde(default)] pub hide_from_all: bool,
	#[serde(default)] pub in_store: bool,
	#[serde(default)] pub into: Vec<String>,
	#[serde(default)] pub id: i32,
	pub stats: Option<InventoryDataStats>,
	#[serde(default)] pub colloq: String,
	#[serde(default)] pub maps: HashMap<String, bool>,
	#[serde(default)] pub special_recipe: i32,
	pub image: Option<Image>,
	#[serde(default)] pub description: String,
	#[serde(default)] pub tags: Vec<String>,
	#[serde(default)] pub effect: HashMap<String, String>,
	#[serde(default)] pub required_champion: String,
	#[serde(default)] pub from: Vec<String>,
	#[serde(default)] pub group: String,
	#[serde(default)] pub consume_on_full: bool,
	#[serde(default)] pub name: String,
	#[serde(default)] pub consumed: bool,
	#[serde(default)] pub sanitized_description: String,
	#[serde(default)] pub depth: i32,
	#[serde(default)] pub stacks: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gold {
	#[serde(default)] pub sell: i32,
	#[serde(default)] pub total: i32,
	#[serde(default)] pub base: i32,
	#[serde(default)] pub purchasable: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDataStats {
	#[serde(default)] pub percent_crit_damage_mod: f64,
	#[serde(default)] pub percent_spell_block_mod: f64,
	#[serde(default)] pub percent_hp_regen_mod: f64,
	#[serde(default)] pub percent_movement_speed_mod: f64,
	#[serde(default)] pub flat_spell_block_mod: f64,
	#[serde(default)] pub flat_crit_damage_mod: f64,
	#[serde(default)] pub flat_energy_pool_mod: f64,
	#[serde(default)] pub percent_life_steal_mod: f64,
	#[serde(default)] pub flat_mp_pool_mod: f64,
	#[serde(default)] pub flat_movement_speed_mod: f64,
	#[serde(default)] pub percent_attack_speed_mod: f64,
	#[serde(default)] pub flat_block_mod: f64,
	#[serde(default)] pub percent_block_mod: f64,
	#[serde(default)] pub flat_energy_regen_mod: f64,
	#[serde(default)] pub percent_spell_vamp_mod: f64,
	#[serde(default)] pub flat_mp_regen_mod: f64,
	#[serde(default)] pub percent_dodge_mod: f64,
	#[serde(default)] pub flat_attack_speed_mod: f64,
	#[serde(default)] pub flat_armor_mod: f64,
	#[serde(default)] pub flat_hp_regen_mod: f64,
	#[serde(default)] pub percent_magic_damage_mod: f64,
	#[serde(default)] pub percent_mp_pool_mod: f64,
	#[serde(default)] pub flat_magic_damage_mod: f64,
	#[serde(default)] pub percent_mp_regen_mod: f64,
	#[serde(default)] pub percent_physical_damage_mod: f64,
	#[serde(default)] pub flat_physical_damage_mod: f64,
	#[serde(default)] pub percent_hp_pool_mod: f64,
	#[serde(default)] pub percent_armor_mod: f64,
	#[serde(default)] pub percent_crit_chance_mod: f64,
	#[serde(default)] pub percent_exp_bonus: f64,
	#[serde(default)] pub flat_hp_pool_mod: f64,
	#[serde(default)] pub flat_crit_chance_mod: f64,
	#[serde(default)] pub flat_exp_bonus: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
	#[serde(default)] pub max_group_ownable: String,
	#[serde(default)] pub key: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageStrings {
	#[serde(default)] pub data: HashMap<String, String>,
	#[serde(default)] pub version: String,
	#[serde(default)] pub strings_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
	#[serde(default)] pub data: HashMap<String, MapDetails>,
	#[serde(default)] pub version: String,
	#[serde(default)] pub data_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapDetails {
	#[serde(default)] pub map_name: String,
	pub image: Option<Image>,
	#[serde(default)] pub map_id: i64,
	#[serde(default)] pub unpurchasable_item_list: Vec<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasteryList {
	#[serde(default)] pub data: HashMap<String, Mastery>,
	#[serde(default)] pub version: String,
	pub tree: Option<MasteryTree>,
	#[serde(default)] pub masteries_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasteryTree {
	#[serde(default)] pub resolve: Vec<MasteryTreeList>,
	#[serde(default)] pub ferocity: Vec<MasteryTreeList>,
	#[serde(default)] pub cunning: Vec<MasteryTreeList>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasteryTreeList {
	#[serde(default)] pub mastery_tree_items: Vec<MasteryTreeItem>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasteryTreeItem {
	#[serde(default)] pub mastery_id: i32,
	#[serde(default)] pub prereq: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mastery {
	#[serde(default)] pub prereq: String,
	#[serde(default)] pub mastery_tree: String,
	#[serde(default)] pub name: String,
	#[serde(default)] pub ranks: i32,
	pub image: Option<Image>,
	#[serde(default)] pub sanitized_description: Vec<String>,
	#[serde(default)] pub id: i32,
	#[serde(default)] pub description: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileIconData {
	#[serde(default)] pub data: HashMap<String, ProfileIconDetails>,
	#[serde(default)] pub version: String,
	#[serde(default)] pub data_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileIconDetails {
	pub image: Image,
	#[serde(default)] pub id: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Realm {
	#[serde(default)] pub lg: String,
	#[serde(default)] pub dd: String,
	#[serde(default)] pub l: String,
	#[serde(default)] pub n: HashMap<String, String>,
	#[serde(default)] pub profileiconmax: i32,
	#[serde(default)] pub store: String,
	#[serde(default)] pub v: String,
	#[serde(default)] pub cdn: String,
	#[serde(default)] pub css: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneList {
	#[serde(default)] pub data: HashMap<String, Rune>,
	#[serde(default)] pub version: String,
	#[serde(default)] pub data_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
	#[serde(default)] pub stats: HashMap<String, f64>,
	#[serde(default)] pub name: String,
	#[serde(default)] pub tags: Vec<String>,
	pub image: Option<Image>,
	#[serde(default)] pub sanitized_description: String,
	pub rune: MetaData,
	#[serde(default)] pub id: i32,
	#[serde(default)] pub description: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
	#[serde(default)] pub tier: String,
	#[serde(default)] pub rune_type: String,
	#[serde(default)] pub is_rune: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpellList {
	#[serde(default)] pub data: HashMap<String, SummonerSpell>,
	#[serde(default)] pub version: String,
	#[serde(default)] pub data_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
	#[serde(default)] pub vars: Vec<SpellVars>,
	pub image: Option<Image>,
	#[serde(default)] pub cost_burn: String,
	#[serde(default)] pub cooldown: Vec<f64>,
	#[serde(default)] pub effect_burn: Vec<String>,
	#[serde(default)] pub id: i32,
	#[serde(default)] pub cooldown_burn: String,
	#[serde(default)] pub tooltip: String,
	#[serde(default)] pub maxrank: i32,
	#[serde(default)] pub range_burn: String,
	#[serde(default)] pub description: String,
	#[serde(default)] pub effect: Vec<Vec<f64>>,
	#[serde(default)] pub key: String,
	pub leveltip: Option<LevelTip>,
	#[serde(default)] pub modes: Vec<String>,
	#[serde(default)] pub resource: String,
	#[serde(default)] pub name: String,
	#[serde(default)] pub cost_type: String,
	#[serde(default)] pub sanitized_description: String,
	#[serde(default)] pub sanitized_tooltip: String,
	pub range: Option<Range>,
	#[serde(default)] pub cost: Vec<i32>,
	#[serde(default)] pub summoner_level: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Range {
	SelfTarget,
	Range(Vec<i32>),
}
impl<'de> de::Deserialize<'de> for Range {
	fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Range, D::Error> {
		struct RangeVisitor;
		impl<'de> de::Visitor<'de> for RangeVisitor {
			type Value = Range;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("string \"self\" or list of integers")
			}

			fn visit_seq<V>(self, mut seq: V) -> Result<Range, V::Error>
			where
				V: de::SeqAccess<'de>,
			{
				let mut ranges = Vec::with_capacity(seq.size_hint().unwrap_or(0));
				while let Some(r) = seq.next_element()? {
					ranges.push(r);
				}
				Ok(Range::Range(ranges))
			}

			fn visit_str<E: de::Error>(self, value: &str) -> Result<Range, E> {
				match value {
					"self" => Ok(Range::SelfTarget),
					_ => Err(de::Error::invalid_value(de::Unexpected::Str(value), &self)),
				}
			}
		}

		deserializer.deserialize_str(RangeVisitor)
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShardStatus {
	#[serde(default)] pub name: String,
	#[serde(default)] pub region_tag: String,
	#[serde(default)] pub hostname: String,
	#[serde(default)] pub services: Vec<Service>,
	#[serde(default)] pub slug: String,
	#[serde(default)] pub locales: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Service {
	#[serde(default)] pub status: String,
	#[serde(default)] pub incidents: Vec<Incident>,
	#[serde(default)] pub name: String,
	#[serde(default)] pub slug: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Incident {
	#[serde(default)] pub active: bool,
	#[serde(default)] pub created_at: String,
	#[serde(default)] pub id: i64,
	#[serde(default)] pub updates: Vec<Message>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Message {
	#[serde(default)] pub severity: String,
	#[serde(default)] pub author: String,
	#[serde(default)] pub created_at: String,
	#[serde(default)] pub translations: Vec<Translation>,
	#[serde(default)] pub updated_at: String,
	#[serde(default)] pub content: String,
	#[serde(default)] pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Translation {
	#[serde(default)] pub locale: String,
	#[serde(default)] pub content: String,
	#[serde(default)] pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MasteryPages {
	#[serde(default)] pub pages: Vec<MasteryPage>,
	#[serde(default)] pub summoner_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MasteryPage {
	#[serde(default)] pub current: bool,
	#[serde(default)] pub masteries: Vec<MasterySelection>,
	#[serde(default)] pub name: String,
	#[serde(default)] pub id: i64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct MasterySelection {
	#[serde(default)] pub id: i32,
	#[serde(default)] pub rank: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Match {
	#[serde(default)] pub season_id: i32,
	#[serde(default)] pub queue_id: i32,
	#[serde(default)] pub game_id: i64,
	#[serde(default)] pub participant_identities: Vec<ParticipantIdentity>,
	#[serde(default)] pub game_version: String,
	#[serde(default)] pub platform_id: String,
	#[serde(default)] pub game_mode: String,
	#[serde(default)] pub map_id: i32,
	#[serde(default)] pub game_type: String,
	#[serde(default)] pub teams: Vec<TeamStats>,
	#[serde(default)] pub participants: Vec<Participant>,
	#[serde(default)] pub game_duration: i64,
	#[serde(default)] pub game_creation: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ParticipantIdentity {
	pub player: Player,
	#[serde(default)] pub participant_id: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Player {
	#[serde(default)] pub current_platform_id: String,
	#[serde(default)] pub summoner_name: String,
	#[serde(default)] pub match_history_uri: String,
	#[serde(default)] pub platform_id: String,
	#[serde(default)] pub current_account_id: i64,
	#[serde(default)] pub profile_icon: i32,
	#[serde(default)] pub summoner_id: i64,
	#[serde(default)] pub account_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeamStats {
	#[serde(default)] pub first_dragon: bool,
	#[serde(default)] pub first_inhibitor: bool,
	#[serde(default)] pub bans: Vec<TeamBans>,
	#[serde(default)] pub baron_kills: i32,
	#[serde(default)] pub first_rift_herald: bool,
	#[serde(default)] pub first_baron: bool,
	#[serde(default)] pub rift_herald_kills: i32,
	#[serde(default)] pub first_blood: bool,
	#[serde(default)] pub team_id: i32,
	#[serde(default)] pub first_tower: bool,
	#[serde(default)] pub vilemaw_kills: i32,
	#[serde(default)] pub inhibitor_kills: i32,
	#[serde(default)] pub tower_kills: i32,
	#[serde(default)] pub dominion_victory_score: i32,
	#[serde(default)] pub win: String,
	#[serde(default)] pub dragon_kills: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeamBans {
	#[serde(default)] pub pick_turn: i32,
	#[serde(default)] pub champion_id: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Participant {
	pub stats: ParticipantStats,
	#[serde(default)] pub participant_id: i32,
	#[serde(default)] pub runes: Vec<RuneSelection>,
	pub timeline: ParticipantTimeline,
	#[serde(default)] pub team_id: i32,
	#[serde(default)] pub spell1_id: i32,
	#[serde(default)] pub spell2_id: i32,
	#[serde(default)] pub masteries: Vec<MasterySelection>,
	#[serde(default)] pub highest_achieved_season_tier: String,
	#[serde(default)] pub champion_id: i32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct ParticipantStats {
	#[serde(default)] pub physical_damage_dealt: i64,
	#[serde(default)] pub neutral_minions_killed_team_jungle: i32,
	#[serde(default)] pub magic_damage_dealt: i64,
	#[serde(default)] pub total_player_score: i32,
	#[serde(default)] pub deaths: i32,
	#[serde(default)] pub win: bool,
	#[serde(default)] pub neutral_minions_killed_enemy_jungle: i32,
	#[serde(default)] pub altars_captured: i32,
	#[serde(default)] pub largest_critical_strike: i32,
	#[serde(default)] pub total_damage_dealt: i64,
	#[serde(default)] pub magic_damage_dealt_to_champions: i64,
	#[serde(default)] pub vision_wards_bought_in_game: i32,
	#[serde(default)] pub damage_dealt_to_objectives: i64,
	#[serde(default)] pub largest_killing_spree: i32,
	#[serde(default)] pub quadra_kills: i32,
	#[serde(default)] pub team_objective: i32,
	#[serde(default)] pub total_time_crowd_control_dealt: i32,
	#[serde(default)] pub i64est_time_spent_living: i32,
	#[serde(default)] pub wards_killed: i32,
	#[serde(default)] pub first_tower_assist: bool,
	#[serde(default)] pub first_tower_kill: bool,
	#[serde(default)] pub item0: i32,
	#[serde(default)] pub item1: i32,
	#[serde(default)] pub item2: i32,
	#[serde(default)] pub item3: i32,
	#[serde(default)] pub item4: i32,
	#[serde(default)] pub item5: i32,
	#[serde(default)] pub item6: i32,
	#[serde(default)] pub first_blood_assist: bool,
	#[serde(default)] pub vision_score: i64,
	#[serde(default)] pub wards_placed: i32,
	#[serde(default)] pub turret_kills: i32,
	#[serde(default)] pub triple_kills: i32,
	#[serde(default)] pub damage_self_mitigated: i64,
	#[serde(default)] pub champ_level: i32,
	#[serde(default)] pub node_neutralize_assist: i32,
	#[serde(default)] pub first_inhibitor_kill: bool,
	#[serde(default)] pub gold_earned: i32,
	#[serde(default)] pub magical_damage_taken: i64,
	#[serde(default)] pub kills: i32,
	#[serde(default)] pub double_kills: i32,
	#[serde(default)] pub node_capture_assist: i32,
	#[serde(default)] pub true_damage_taken: i64,
	#[serde(default)] pub node_neutralize: i32,
	#[serde(default)] pub first_inhibitor_assist: bool,
	#[serde(default)] pub assists: i32,
	#[serde(default)] pub unreal_kills: i32,
	#[serde(default)] pub neutral_minions_killed: i32,
	#[serde(default)] pub objective_player_score: i32,
	#[serde(default)] pub combat_player_score: i32,
	#[serde(default)] pub damage_dealt_to_turrets: i64,
	#[serde(default)] pub altars_neutralized: i32,
	#[serde(default)] pub physical_damage_dealt_to_champions: i64,
	#[serde(default)] pub gold_spent: i32,
	#[serde(default)] pub true_damage_dealt: i64,
	#[serde(default)] pub true_damage_dealt_to_champions: i64,
	#[serde(default)] pub participant_id: i32,
	#[serde(default)] pub penta_kills: i32,
	#[serde(default)] pub total_heal: i64,
	#[serde(default)] pub total_minions_killed: i32,
	#[serde(default)] pub first_blood_kill: bool,
	#[serde(default)] pub node_capture: i32,
	#[serde(default)] pub largest_multi_kill: i32,
	#[serde(default)] pub sight_wards_bought_in_game: i32,
	#[serde(default)] pub total_damage_dealt_to_champions: i64,
	#[serde(default)] pub total_units_healed: i32,
	#[serde(default)] pub inhibitor_kills: i32,
	#[serde(default)] pub total_score_rank: i32,
	#[serde(default)] pub total_damage_taken: i64,
	#[serde(default)] pub killing_sprees: i32,
	#[serde(default)] pub time_c_cing_others: i64,
	#[serde(default)] pub physical_damage_taken: i64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct RuneSelection {
	#[serde(default)] pub rune_id: i32,
	#[serde(default)] pub rank: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ParticipantTimeline {
	#[serde(default)] pub lane: String,
	#[serde(default)] pub participant_id: i32,
	#[serde(default)] pub role: String,
	#[serde(default)] pub cs_diff_per_min_deltas: HashMap<String, f64>,
	#[serde(default)] pub gold_per_min_deltas: HashMap<String, f64>,
	#[serde(default)] pub xp_diff_per_min_deltas: HashMap<String, f64>,
	#[serde(default)] pub creeps_per_min_deltas: HashMap<String, f64>,
	#[serde(default)] pub xp_per_min_deltas: HashMap<String, f64>,
	#[serde(default)] pub damage_taken_diff_per_min_deltas: HashMap<String, f64>,
	#[serde(default)] pub damage_taken_per_min_deltas: HashMap<String, f64>,
}
