use {dto, request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		method_limits: &'a MethodLimits,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits }
	}

	/// "Retrieve all champions."
	///
	/// **Endpoint**: `/lol/platform/v3/champions`
	pub fn get(&self) -> Result<Vec<dto::platform::Champion>, StatusCode> {
		let path = "/lol/platform/v3/champions";
		request::<dto::platform::ChampionList, _>(
			self.region,
			&self.key,
			path,
			Some(&self.app_limit),
			&self.method_limits.get,
		).map(|x| x.champions)
	}

	/// "Retrieve champion by ID."
	///
	/// **Endpoint**: `/lol/platform/v3/champions/{id}`
	pub fn get_id(&self, id: i64) -> Result<dto::platform::Champion, StatusCode> {
		let path = format!("/lol/platform/v3/champions/{id}", id = id);
		request(self.region, &self.key, &path, Some(&self.app_limit), &self.method_limits.get_id)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	get: Mutex<Option<GCRA>>,
	get_id: Mutex<Option<GCRA>>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: Mutex::default(), get_id: Mutex::default() }
	}
}
