use {dto, request, QueueType, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	get_master_league_limit: &'a Mutex<Option<GCRA>>,
	queue: QueueType,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		get_master_league_limit: &'a Mutex<Option<GCRA>>,
		queue: QueueType,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			get_master_league_limit: get_master_league_limit,
			queue: queue,
		}
	}

	/// "Get the master league for a given queue."
	///
	/// **Endpoint**: `/lol/league/v3/masterleagues/by-queue/{queue}`
	pub fn get(&self) -> Result<dto::LeagueList, StatusCode> {
		let path = format!("/lol/league/v3/masterleagues/by-queue/{queue}", queue = self.queue.to_str());
		request(self.region, &self.key, &path, &self.app_limit, self.get_master_league_limit)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
