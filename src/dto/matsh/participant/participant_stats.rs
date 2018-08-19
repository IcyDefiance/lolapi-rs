#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantStats {
	#[serde(default)]
	pub altars_captured: i32,

	#[serde(default)]
	pub altars_neutralized: i32,

	#[serde(default)]
	pub assists: i32,

	#[serde(default)]
	pub champ_level: i32,

	#[serde(default)]
	pub combat_player_score: i32,

	#[serde(default)]
	pub damage_dealt_to_objectives: i64,

	#[serde(default)]
	pub damage_dealt_to_turrets: i64,

	#[serde(default)]
	pub damage_self_mitigated: i64,

	#[serde(default)]
	pub deaths: i32,

	#[serde(default)]
	pub double_kills: i32,

	pub first_blood_assist: bool,

	pub first_blood_kill: bool,

	pub first_inhibitor_assist: bool,

	pub first_inhibitor_kill: bool,

	pub first_tower_assist: bool,

	pub first_tower_kill: bool,

	#[serde(default)]
	pub gold_earned: i32,

	#[serde(default)]
	pub gold_spent: i32,

	#[serde(default)]
	pub inhibitor_kills: i32,

	#[serde(default)]
	pub item0: i32,

	#[serde(default)]
	pub item1: i32,

	#[serde(default)]
	pub item2: i32,

	#[serde(default)]
	pub item3: i32,

	#[serde(default)]
	pub item4: i32,

	#[serde(default)]
	pub item5: i32,

	#[serde(default)]
	pub item6: i32,

	#[serde(default)]
	pub killing_sprees: i32,

	#[serde(default)]
	pub kills: i32,

	#[serde(default)]
	pub largest_critical_strike: i32,

	#[serde(default)]
	pub largest_killing_spree: i32,

	#[serde(default)]
	pub largest_multi_kill: i32,

	#[serde(default)]
	pub longest_time_spent_living: i32,

	#[serde(default)]
	pub magical_damage_taken: i64,

	#[serde(default)]
	pub magic_damage_dealt: i64,

	#[serde(default)]
	pub magic_damage_dealt_to_champions: i64,

	#[serde(default)]
	pub neutral_minions_killed: i32,

	#[serde(default)]
	pub neutral_minions_killed_enemy_jungle: i32,

	#[serde(default)]
	pub neutral_minions_killed_team_jungle: i32,

	#[serde(default)]
	pub node_capture: i32,

	#[serde(default)]
	pub node_capture_assist: i32,

	#[serde(default)]
	pub node_neutralize: i32,

	#[serde(default)]
	pub node_neutralize_assist: i32,

	#[serde(default)]
	pub objective_player_score: i32,

	#[serde(default)]
	pub participant_id: i32,

	#[serde(default)]
	pub penta_kills: i32,

	#[serde(default)]
	pub perk0: i32,

	#[serde(default)]
	pub perk0_var1: i32,

	#[serde(default)]
	pub perk0_var2: i32,

	#[serde(default)]
	pub perk0_var3: i32,

	#[serde(default)]
	pub perk1: i32,

	#[serde(default)]
	pub perk1_var1: i32,

	#[serde(default)]
	pub perk1_var2: i32,

	#[serde(default)]
	pub perk1_var3: i32,

	#[serde(default)]
	pub perk2: i32,

	#[serde(default)]
	pub perk2_var1: i32,

	#[serde(default)]
	pub perk2_var2: i32,

	#[serde(default)]
	pub perk2_var3: i32,

	#[serde(default)]
	pub perk3: i32,

	#[serde(default)]
	pub perk3_var1: i32,

	#[serde(default)]
	pub perk3_var2: i32,

	#[serde(default)]
	pub perk3_var3: i32,

	#[serde(default)]
	pub perk4: i32,

	#[serde(default)]
	pub perk4_var1: i32,

	#[serde(default)]
	pub perk4_var2: i32,

	#[serde(default)]
	pub perk4_var3: i32,

	#[serde(default)]
	pub perk5: i32,

	#[serde(default)]
	pub perk5_var1: i32,

	#[serde(default)]
	pub perk5_var2: i32,

	#[serde(default)]
	pub perk5_var3: i32,

	#[serde(default)]
	pub perk_primary_style: i32,

	#[serde(default)]
	pub perk_sub_style: i32,

	#[serde(default)]
	pub physical_damage_dealt: i64,

	#[serde(default)]
	pub physical_damage_dealt_to_champions: i64,

	#[serde(default)]
	pub physical_damage_taken: i64,

	#[serde(default)]
	pub player_score0: i32,

	#[serde(default)]
	pub player_score1: i32,

	#[serde(default)]
	pub player_score2: i32,

	#[serde(default)]
	pub player_score3: i32,

	#[serde(default)]
	pub player_score4: i32,

	#[serde(default)]
	pub player_score5: i32,

	#[serde(default)]
	pub player_score6: i32,

	#[serde(default)]
	pub player_score7: i32,

	#[serde(default)]
	pub player_score8: i32,

	#[serde(default)]
	pub player_score9: i32,

	#[serde(default)]
	pub quadra_kills: i32,

	#[serde(default)]
	pub sight_wards_bought_in_game: i32,

	#[serde(default)]
	pub team_objective: i32,

	#[serde(default)]
	pub time_ccing_others: i64,

	#[serde(default)]
	pub total_damage_dealt: i64,

	#[serde(default)]
	pub total_damage_dealt_to_champions: i64,

	#[serde(default)]
	pub total_damage_taken: i64,

	#[serde(default)]
	pub total_heal: i64,

	#[serde(default)]
	pub total_minions_killed: i32,

	#[serde(default)]
	pub total_player_score: i32,

	#[serde(default)]
	pub total_score_rank: i32,

	#[serde(default)]
	pub total_time_crowd_control_dealt: i32,

	#[serde(default)]
	pub total_units_healed: i32,

	#[serde(default)]
	pub triple_kills: i32,

	#[serde(default)]
	pub true_damage_dealt: i64,

	#[serde(default)]
	pub true_damage_dealt_to_champions: i64,

	#[serde(default)]
	pub true_damage_taken: i64,

	#[serde(default)]
	pub turret_kills: i32,

	#[serde(default)]
	pub unreal_kills: i32,

	#[serde(default)]
	pub vision_score: i64,

	#[serde(default)]
	pub vision_wards_bought_in_game: i32,

	#[serde(default)]
	pub wards_killed: i32,

	#[serde(default)]
	pub wards_placed: i32,

	pub win: bool,
}
