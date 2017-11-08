pub mod shard_data;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	method_limits: &'a mut MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: &'a K, method_limits: &'a mut MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	pub fn shard_data(&mut self) -> shard_data::Subclient<K> {
		shard_data::Subclient::new(self.region, self.key, &mut self.method_limits.shard_data)
	}
}

pub(super) struct MethodLimits {
	shard_data: shard_data::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { shard_data: shard_data::MethodLimits::new() }
	}
}
