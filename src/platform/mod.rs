pub mod champions;
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	get_champions_limit: &'a Mutex<Option<GCRA>>,
	get_champion_limit: &'a Mutex<Option<GCRA>>,
}
impl<'a, K: Display + Clone> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		get_champions_limit: &'a Mutex<Option<GCRA>>,
		get_champion_limit: &'a Mutex<Option<GCRA>>,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			get_champions_limit: get_champions_limit,
			get_champion_limit: get_champion_limit,
		}
	}

	pub fn champions(&self) -> champions::Subclient<K> {
		champions::Subclient::new(
			self.region,
			self.key.clone(),
			&self.app_limit,
			&self.get_champions_limit,
			&self.get_champion_limit,
		)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
