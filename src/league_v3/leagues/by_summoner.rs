use {dto, request, StatusCode};
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limit: &'a mut Vec<LeakyBucket>,
	method_limits: &'a mut MethodLimits,
	summoner_id: i64,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limit: &'a mut Vec<LeakyBucket>,
		method_limits: &'a mut MethodLimits,
		summoner_id: i64,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits, summoner_id: summoner_id }
	}

	/// "Get leagues in all queues for a given summoner ID."
	///
	/// **Endpoint**: `/lol/league/v3/leagues/by-summoner/{summonerId}`
	pub fn get(&mut self) -> Result<Vec<dto::LeagueList>, StatusCode> {
		let path = format!("/lol/league/v3/leagues/by-summoner/{summoner_id}", summoner_id = self.summoner_id);
		request(self.region, self.key, &path, self.app_limit, &mut self.method_limits.get)
	}
}

pub(super) struct MethodLimits {
	get: Vec<LeakyBucket>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: vec![] }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT.lock().unwrap().league_v3().leagues().by_summoner(24338059).get().unwrap();
	}
}
