mod game_mode;
mod game_type;
mod participant;
mod participant_identity;
mod team_stats;

pub use self::game_mode::GameMode;
pub use self::game_type::GameType;
pub use self::participant::{ Mastery, Participant, ParticipantStats, ParticipantTimeline, Rune };
pub use self::participant_identity::{ ParticipantIdentity, Player };
pub use self::team_stats::{ TeamBans, TeamStats };

use chrono::{ NaiveDateTime, naive::serde::ts_milliseconds };
use dto::Platform;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
	#[serde(with = "ts_milliseconds")]
	pub game_creation: NaiveDateTime,

	#[serde(default)]
	pub game_duration: i64,

	#[serde(default)]
	pub game_id: i64,

	pub game_mode: GameMode,

	pub game_type: GameType,

	#[serde(default)]
	pub game_version: String,

	#[serde(default)]
	pub map_id: i16,

	#[serde(default)]
	pub participants: Vec<Participant>,

	#[serde(default)]
	pub participant_identities: Vec<ParticipantIdentity>,

	pub platform_id: Platform,

	#[serde(default)]
	pub queue_id: i32,

	#[serde(default)]
	pub season: i16,

	#[serde(default)]
	pub teams: Vec<TeamStats>,
}
