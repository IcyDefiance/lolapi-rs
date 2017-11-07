pub mod shard_data;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: &'a K, method_limits: &'a MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	pub fn shard_data(&self) -> shard_data::Subclient<K> {
		shard_data::Subclient::new(self.region, self.key, &self.method_limits.shard_data)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	shard_data: shard_data::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { shard_data: shard_data::MethodLimits::new() }
	}
}
