pub mod by_queue;
use QueueType;
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

	pub fn by_queue(&mut self, queue: QueueType) -> by_queue::Subclient<K> {
		by_queue::Subclient::new(self.region, self.key, self.app_limit, &mut self.method_limits.by_queue, queue)
	}
}

pub(super) struct MethodLimits {
	by_queue: by_queue::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { by_queue: by_queue::MethodLimits::new() }
	}
}
