pub mod by_summoner;
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limits: &'a mut Vec<LeakyBucket>,
	method_limits: &'a mut MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limits: &'a mut Vec<LeakyBucket>,
		method_limits: &'a mut MethodLimits,
	) -> Self {
		Self { region: region, key: key, app_limits: app_limits, method_limits: method_limits }
	}

	pub fn by_summoner(&mut self, summoner_id: i64) -> by_summoner::Subclient<K> {
		by_summoner::Subclient::new(
			self.region,
			self.key,
			self.app_limits,
			&mut self.method_limits.by_summoner,
			summoner_id,
		)
	}
}

pub(super) struct MethodLimits {
	by_summoner: by_summoner::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { by_summoner: by_summoner::MethodLimits::new() }
	}
}
