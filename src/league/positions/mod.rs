pub mod by_summoner;
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display + Clone> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		method_limits: &'a MethodLimits,
	) -> Self {
		Self { region: region, key: key, app_limit: app_limit, method_limits: method_limits }
	}

	pub fn by_summoner(&self, summoner_id: i64) -> by_summoner::Subclient<K> {
		by_summoner::Subclient::new(
			self.region,
			self.key.clone(),
			&self.app_limit,
			&self.method_limits.by_summoner,
			summoner_id,
		)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	by_summoner: by_summoner::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { by_summoner: by_summoner::MethodLimits::new() }
	}
}
