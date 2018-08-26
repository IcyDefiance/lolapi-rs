use dto::Lane;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantTimeline {
	#[serde(default)]
	pub creeps_per_min_deltas: HashMap<String, f64>,

	#[serde(default)]
	pub cs_diff_per_min_deltas: HashMap<String, f64>,

	#[serde(default)]
	pub damage_taken_diff_per_min_deltas: HashMap<String, f64>,

	#[serde(default)]
	pub damage_taken_per_min_deltas: HashMap<String, f64>,

	#[serde(default)]
	pub gold_per_min_deltas: HashMap<String, f64>,

	pub lane: Lane,

	#[serde(default)]
	pub participant_id: i32,

	#[serde(default)]
	pub role: String,

	#[serde(default)]
	pub xp_diff_per_min_deltas: HashMap<String, f64>,

	#[serde(default)]
	pub xp_per_min_deltas: HashMap<String, f64>,
}
