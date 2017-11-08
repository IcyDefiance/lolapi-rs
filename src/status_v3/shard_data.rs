use {dto, request, StatusCode};
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	method_limits: &'a mut MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: &'a K, method_limits: &'a mut MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	/// "Get League of Legends status for the given shard."
	///
	/// **Endpoint**: `/lol/status/v3/shard-data`
	pub fn get(&mut self) -> Result<dto::ShardStatus, StatusCode> {
		let path = "/lol/status/v3/shard-data";
		request(self.region, self.key, path, &mut vec![], &mut self.method_limits.get)
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
		::CLIENT.lock().unwrap().status_v3().shard_data().get().unwrap();
	}
}
