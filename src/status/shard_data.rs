use {dto, request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: &'a K, method_limits: &'a MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	/// "Get League of Legends status for the given shard."
	///
	/// **Endpoint**: `/lol/status/v3/shard-data`
	pub fn get(&self) -> Result<dto::ShardStatus, StatusCode> {
		let path = "/lol/status/v3/shard-data";
		request(self.region, self.key, path, None, &self.method_limits.get)
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
		::CLIENT.status().shard_data().get().unwrap();
	}
}
