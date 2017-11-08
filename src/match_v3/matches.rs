use {dto, request, StatusCode};
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limit: &'a mut Vec<LeakyBucket>,
	method_limits: &'a mut MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limit: &'a mut Vec<LeakyBucket>,
		method_limits: &'a mut MethodLimits,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits }
	}

	/// "Retrieve match by ID."
	///
	/// **Endpoint**: `/lol/match/v3/matches/{match_id}`
	pub fn get_id(&mut self, match_id: i64) -> Result<dto::Match, StatusCode> {
		let path = format!("/lol/match/v3/matches/{match_id}", match_id = match_id);
		request(self.region, self.key, &path, self.app_limit, &mut self.method_limits.get_id)
	}
}

pub(super) struct MethodLimits {
	get_id: Vec<LeakyBucket>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get_id: vec![] }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get_id() {
		::CLIENT.lock().unwrap().match_v3().matches().get_id(2631382658).unwrap();
	}
}
