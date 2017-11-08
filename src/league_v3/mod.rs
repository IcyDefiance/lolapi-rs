pub mod challengerleagues;
pub mod leagues;
pub mod masterleagues;
pub mod positions;
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

	pub fn challengerleagues(&mut self) -> challengerleagues::Subclient<K> {
		challengerleagues::Subclient::new(self.region, self.key, self.app_limit, &mut self.method_limits.challengerleagues)
	}

	pub fn leagues(&mut self) -> leagues::Subclient<K> {
		leagues::Subclient::new(self.region, self.key, self.app_limit, &mut self.method_limits.leagues)
	}

	pub fn masterleagues(&mut self) -> masterleagues::Subclient<K> {
		masterleagues::Subclient::new(self.region, self.key, self.app_limit, &mut self.method_limits.masterleagues)
	}

	pub fn positions(&mut self) -> positions::Subclient<K> {
		positions::Subclient::new(self.region, self.key, self.app_limit, &mut self.method_limits.positions)
	}
}

pub(super) struct MethodLimits {
	challengerleagues: challengerleagues::MethodLimits,
	leagues: leagues::MethodLimits,
	masterleagues: masterleagues::MethodLimits,
	positions: positions::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self {
			challengerleagues: challengerleagues::MethodLimits::new(),
			leagues: leagues::MethodLimits::new(),
			masterleagues: masterleagues::MethodLimits::new(),
			positions: positions::MethodLimits::new(),
		}
	}
}
