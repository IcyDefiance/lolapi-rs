pub mod champion_masteries;
pub mod scores;
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	get_champion_masteries_limit: &'a Mutex<Option<GCRA>>,
	get_champion_mastery_limit: &'a Mutex<Option<GCRA>>,
	get_champion_mastery_score_limit: &'a Mutex<Option<GCRA>>,
}
impl<'a, K: Display + Clone> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		get_champion_masteries_limit: &'a Mutex<Option<GCRA>>,
		get_champion_mastery_limit: &'a Mutex<Option<GCRA>>,
		get_champion_mastery_score_limit: &'a Mutex<Option<GCRA>>,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			get_champion_masteries_limit: get_champion_masteries_limit,
			get_champion_mastery_limit: get_champion_mastery_limit,
			get_champion_mastery_score_limit: get_champion_mastery_score_limit,
		}
	}

	pub fn champion_masteries(&self) -> champion_masteries::Subclient<K> {
		champion_masteries::Subclient::new(
			self.region,
			self.key.clone(),
			&self.app_limit,
			&self.get_champion_masteries_limit,
			&self.get_champion_mastery_limit,
		)
	}

	pub fn scores(&self) -> scores::Subclient<K> {
		scores::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.get_champion_mastery_score_limit)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
