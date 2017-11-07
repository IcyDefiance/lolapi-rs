pub mod champion_masteries;
pub mod scores;
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

	pub fn champion_masteries(&self) -> champion_masteries::Subclient<K> {
		champion_masteries::Subclient::new(
			self.region,
			self.key,
			&self.app_limit,
			&self.method_limits.champion_masteries,
		)
	}

	pub fn scores(&self) -> scores::Subclient<K> {
		scores::Subclient::new(self.region, self.key, &self.app_limit, &self.method_limits.scores)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	champion_masteries: champion_masteries::MethodLimits,
	scores: scores::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { champion_masteries: champion_masteries::MethodLimits::new(), scores: scores::MethodLimits::new() }
	}
}
