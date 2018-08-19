mod mastery;
mod participant_stats;
mod participant_timeline;
mod rune;

pub use self::mastery::Mastery;
pub use self::participant_stats::ParticipantStats;
pub use self::participant_timeline::ParticipantTimeline;
pub use self::rune::Rune;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
	#[serde(default)]
	pub champion_id: i32,

	#[serde(default)]
	pub highest_achieved_season_tier: String,

	#[serde(default)]
	pub masteries: Vec<Mastery>,

	#[serde(default)]
	pub participant_id: i32,

	#[serde(default)]
	pub runes: Vec<Rune>,

	#[serde(default)]
	pub spell1_id: i32,

	#[serde(default)]
	pub spell2_id: i32,

	pub stats: ParticipantStats,

	#[serde(default)]
	pub team_id: i32,

	pub timeline: ParticipantTimeline,
}
