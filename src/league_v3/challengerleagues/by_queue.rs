use {dto, request, QueueType, StatusCode};
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limit: &'a mut Vec<LeakyBucket>,
	method_limits: &'a mut MethodLimits,
	queue: QueueType,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limit: &'a mut Vec<LeakyBucket>,
		method_limits: &'a mut MethodLimits,
		queue: QueueType,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits, queue: queue }
	}

	/// "Get the challenger league for a given queue."
	///
	/// **Endpoint**: `/lol/league/v3/challengerleagues/by-queue/{queue}`
	pub fn get(&mut self) -> Result<dto::LeagueList, StatusCode> {
		let path = format!("/lol/league/v3/challengerleagues/by-queue/{queue}", queue = self.queue.to_str());
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
		::CLIENT.lock().unwrap().league_v3().challengerleagues().by_queue(::QueueType::RankedSolo5x5).get().unwrap();
	}
}
