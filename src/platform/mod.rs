pub mod champions;
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limit: &'a Mutex<Option<GCRA>>,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limit: &'a Mutex<Option<GCRA>>,
		method_limits: &'a MethodLimits,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits }
	}

	pub fn champions(&self) -> champions::Subclient<K> {
		champions::Subclient::new(self.region, self.key, &self.app_limit, &self.method_limits.champions)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	champions: champions::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { champions: champions::MethodLimits::new() }
	}
}
