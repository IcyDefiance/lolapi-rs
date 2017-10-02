pub mod challengerleagues;
pub mod leagues;
pub mod masterleagues;
pub mod positions;
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

	pub fn challengerleagues(&self) -> challengerleagues::Subclient<K> {
		challengerleagues::Subclient::new(
			self.region,
			self.key.clone(),
			&self.app_limit,
			&self.method_limits.challengerleagues,
		)
	}

	pub fn leagues(&self) -> leagues::Subclient<K> {
		leagues::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.method_limits.leagues)
	}

	pub fn masterleagues(&self) -> masterleagues::Subclient<K> {
		masterleagues::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.method_limits.masterleagues)
	}

	pub fn positions(&self) -> positions::Subclient<K> {
		positions::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.method_limits.positions)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

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
