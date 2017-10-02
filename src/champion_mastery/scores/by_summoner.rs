use {request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	get_champion_mastery_score_limit: &'a Mutex<Option<GCRA>>,
	summoner_id: i64,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		get_champion_mastery_score_limit: &'a Mutex<Option<GCRA>>,
		summoner_id: i64,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			get_champion_mastery_score_limit: get_champion_mastery_score_limit,
			summoner_id: summoner_id,
		}
	}

	/// "Get a player's total champion mastery score, which is the sum of individual champion mastery levels."
	///
	/// **Endpoint**: `/lol/champion-mastery/v3/scores/by-summoner/{summoner_id}`
	pub fn get(&self) -> Result<i32, StatusCode> {
		let path = format!("/lol/champion-mastery/v3/scores/by-summoner/{summoner_id}", summoner_id = self.summoner_id);
		request(self.region, &self.key, &path, &self.app_limit, self.get_champion_mastery_score_limit)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
