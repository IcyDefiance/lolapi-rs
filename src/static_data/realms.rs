use {dto, request, Locale, StatusCode};
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

	/// "Retrieve realm data"
	///
	/// **Endpoint**: `/lol/static-data/v3/realms`
	pub fn get(&self) -> Result<dto::Realm, StatusCode> {
		let path = "/lol/static-data/v3/realms";
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
		::CLIENT.static_data().profile_icons().get(Some(::Locale::en_US), None).unwrap();
	}
}
