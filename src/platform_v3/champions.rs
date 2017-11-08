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

	/// "Retrieve all champions."
	///
	/// **Endpoint**: `/lol/platform/v3/champions`
	pub fn get(&mut self, free_to_play: bool) -> Result<Vec<dto::ChampionDynamic>, StatusCode> {
		let path = "/lol/platform/v3/champions";
		let mut params = vec![];
		if free_to_play {
			params.push(("freeToPlay", "true"));
		}
		request::<dto::ChampionListDynamic, _>(
			self.region,
			&self.key,
			path,
			self.app_limit,
			&mut self.method_limits.get,
		).map(|x| x.champions)
	}

	/// "Retrieve champion by ID."
	///
	/// **Endpoint**: `/lol/platform/v3/champions/{id}`
	pub fn get_id(&mut self, id: i64) -> Result<dto::ChampionDynamic, StatusCode> {
		let path = format!("/lol/platform/v3/champions/{id}", id = id);
		request(self.region, self.key, &path, self.app_limit, &mut self.method_limits.get_id)
	}
}

pub(super) struct MethodLimits {
	get: Vec<LeakyBucket>,
	get_id: Vec<LeakyBucket>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: vec![], get_id: vec![] }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT.lock().unwrap().platform_v3().champions().get(false).unwrap();
	}

	#[test]
	fn get_id() {
		::CLIENT.lock().unwrap().platform_v3().champions().get_id(266).unwrap();
	}
}
