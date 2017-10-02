pub mod by_queue;
use QueueType;
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	get_challenger_league_limit: &'a Mutex<Option<GCRA>>,
}
impl<'a, K: Display + Clone> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		get_challenger_league_limit: &'a Mutex<Option<GCRA>>,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			get_challenger_league_limit: get_challenger_league_limit,
		}
	}

	pub fn by_queue(&self, queue: QueueType) -> by_queue::Subclient<K> {
		by_queue::Subclient::new(
			self.region,
			self.key.clone(),
			&self.app_limit,
			&self.get_challenger_league_limit,
			queue,
		)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
