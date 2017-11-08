pub mod champions;
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

	pub fn champions(&mut self) -> champions::Subclient<K> {
		champions::Subclient::new(self.region, self.key, self.app_limit, &mut self.method_limits.champions)
	}
}

pub(super) struct MethodLimits {
	champions: champions::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { champions: champions::MethodLimits::new() }
	}
}
