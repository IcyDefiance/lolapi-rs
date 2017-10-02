use {dto, request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	get_summoner_positions_limit: &'a Mutex<Option<GCRA>>,
	summoner_id: i64,
}
impl<'a, K: Display + Clone> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		get_summoner_positions_limit: &'a Mutex<Option<GCRA>>,
		summoner_id: i64,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			get_summoner_positions_limit: get_summoner_positions_limit,
			summoner_id: summoner_id,
		}
	}

	/// "Get leagues in all queues for a given summoner ID."
	///
	/// **Endpoint**: `/lol/league/v3/leagues/by-summoner/{summonerId}`
	pub fn get(&self) -> Result<Vec<dto::LeaguePosition>, StatusCode> {
		let path = format!("/lol/league/v3/leagues/by-summoner/{summoner_id}", summoner_id = self.summoner_id);
		request(self.region, &self.key, &path, &self.app_limit, self.get_summoner_positions_limit)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
