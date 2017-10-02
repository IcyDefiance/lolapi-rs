pub mod champions;
pub mod items;
use std::fmt::Display;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display + Clone> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: K, method_limits: &'a MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	pub fn champions(&self) -> champions::Subclient<K> {
		champions::Subclient::new(self.region, self.key.clone(), &self.method_limits.champions)
	}

	pub fn items(&self) -> items::Subclient<K> {
		items::Subclient::new(self.region, self.key.clone(), &self.method_limits.items)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	champions: champions::MethodLimits,
	items: items::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { champions: champions::MethodLimits::new(), items: items::MethodLimits::new() }
	}
}
