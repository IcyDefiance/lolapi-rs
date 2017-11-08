pub mod by_champion;
use {dto, request, StatusCode};
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limits: &'a mut Vec<LeakyBucket>,
	method_limits: &'a mut MethodLimits,
	summoner_id: i64,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limits: &'a mut Vec<LeakyBucket>,
		method_limits: &'a mut MethodLimits,
		summoner_id: i64,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limits: app_limits,
			method_limits: method_limits,
			summoner_id: summoner_id,
		}
	}

	/// "Get all champion mastery entries sorted by number of champion points descending."
	///
	/// **Endpoint**: `/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}`
	pub fn get(&mut self) -> Result<Vec<dto::ChampionMastery>, StatusCode> {
		let path = format!(
			"/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}",
			summoner_id = self.summoner_id
		);
		request(self.region, self.key, &path, self.app_limits, &mut self.method_limits.get)
	}

	pub fn by_champion(&mut self, champion_id: i64) -> by_champion::Subclient<K> {
		by_champion::Subclient::new(
			self.region,
			self.key,
			self.app_limits,
			&mut self.method_limits.by_champion,
			self.summoner_id,
			champion_id,
		)
	}
}

pub(super) struct MethodLimits {
	get: Vec<LeakyBucket>,
	by_champion: by_champion::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: vec![], by_champion: by_champion::MethodLimits::new() }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT.lock().unwrap().champion_mastery_v3().champion_masteries().by_summoner(24338059).get().unwrap();
	}
}
