use {request, StatusCode};
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

	/// "Get a player's total champion mastery score, which is the sum of individual champion mastery levels."
	///
	/// **Endpoint**: `/lol/champion-mastery/v3/scores/by-summoner/{summoner_id}`
	pub fn get(&self) -> Result<i32, StatusCode> {
		let path = format!("/lol/champion-mastery/v3/scores/by-summoner/{summoner_id}", summoner_id = self.summoner_id);
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
		::CLIENT.champion_mastery_v3().scores().by_summoner(24338059).get().unwrap();
	}
}
