pub mod by_account;
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

	pub fn by_account(&mut self, account_id: i64) -> by_account::Subclient<K> {
		by_account::Subclient::new(
			self.region,
			self.key,
			self.app_limit,
			&mut self.method_limits.by_account,
			account_id,
		)
	}
}

pub(super) struct MethodLimits {
	by_account: by_account::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { by_account: by_account::MethodLimits::new() }
	}
}
