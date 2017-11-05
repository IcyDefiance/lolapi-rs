use {dto, request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limit: &'a Mutex<Option<GCRA>>,
	method_limits: &'a MethodLimits,
	summoner_id: i64,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limit: &'a Mutex<Option<GCRA>>,
		method_limits: &'a MethodLimits,
		summoner_id: i64,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits, summoner_id: summoner_id }
	}

	/// "Get leagues in all queues for a given summoner ID."
	///
	/// **Endpoint**: `/lol/league/v3/leagues/by-summoner/{summonerId}`
	pub fn get(&self) -> Result<Vec<dto::LeagueList>, StatusCode> {
		let path = format!("/lol/league/v3/leagues/by-summoner/{summoner_id}", summoner_id = self.summoner_id);
		request(self.region, self.key, &path, Some(&self.app_limit), &self.method_limits.get)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	get: Mutex<Option<GCRA>>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: Mutex::default() }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT.league_v3().leagues().by_summoner(24338059).get().unwrap();
	}
}
