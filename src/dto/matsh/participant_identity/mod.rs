mod player;

pub use self::player::Player;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentity {
	#[serde(default)]
	pub participant_id: i32,

	pub player: Player,
}
