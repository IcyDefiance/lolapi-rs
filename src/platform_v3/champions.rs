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

	/// "Retrieve all champions."
	///
	/// **Endpoint**: `/lol/platform/v3/champions`
	pub fn get(&self, free_to_play: bool) -> Result<Vec<dto::ChampionDynamic>, StatusCode> {
		let path = "/lol/platform/v3/champions";
		let mut params = vec![];
		if free_to_play {
			params.push(("freeToPlay", "true"));
		}
		request::<dto::ChampionListDynamic, _>(
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
	pub fn get_id(&self, id: i64) -> Result<dto::ChampionDynamic, StatusCode> {
		let path = format!("/lol/platform/v3/champions/{id}", id = id);
		request(self.region, self.key, &path, Some(&self.app_limit), &self.method_limits.get_id)
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

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT.platform_v3().champions().get(false).unwrap();
	}

	#[test]
	fn get_id() {
		::CLIENT.platform_v3().champions().get_id(266).unwrap();
	}
}
