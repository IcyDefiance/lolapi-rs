use {dto, request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limit: &'a Mutex<GCRA>,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limit: &'a Mutex<GCRA>,
		method_limits: &'a MethodLimits,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits }
	}

	/// "Retrieve match by ID."
	///
	/// **Endpoint**: `/lol/match/v3/matches/{match_id}`
	pub fn get_id(&self, match_id: i64) -> Result<dto::Match, StatusCode> {
		let path = format!("/lol/match/v3/matches/{match_id}", match_id = match_id);
		request(self.region, self.key, &path, Some(&self.app_limit), &self.method_limits.get_id)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	get_id: Mutex<Option<GCRA>>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get_id: Mutex::default() }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get_id() {
		::CLIENT.match_v3().matches().get_id(2631382658).unwrap();
	}
}
