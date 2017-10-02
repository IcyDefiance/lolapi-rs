use {dto, request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	method_limits: &'a MethodLimits,
	summoner_id: i64,
	champion_id: i64,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		method_limits: &'a MethodLimits,
		summoner_id: i64,
		champion_id: i64,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			method_limits: method_limits,
			summoner_id: summoner_id,
			champion_id: champion_id,
		}
	}

	/// "Get a champion mastery by player ID and champion ID."
	///
	/// **Endpoint**: `/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}/by-champion/{champion_id}`
	pub fn get(&self) -> Result<dto::ChampionMastery, StatusCode> {
		let path = format!(
			"/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}/by-champion/{champion_id}",
			summoner_id = self.summoner_id,
			champion_id = self.champion_id
		);
		request(self.region, &self.key, &path, &self.app_limit, &self.method_limits.get)
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
