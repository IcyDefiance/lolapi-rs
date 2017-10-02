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
	get_challenger_league_limit: &'a Mutex<Option<GCRA>>,
	get_summoner_leagues_limit: &'a Mutex<Option<GCRA>>,
	get_master_league_limit: &'a Mutex<Option<GCRA>>,
	get_summoner_positions_limit: &'a Mutex<Option<GCRA>>,
}
impl<'a, K: Display + Clone> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		get_challenger_league_limit: &'a Mutex<Option<GCRA>>,
		get_summoner_leagues_limit: &'a Mutex<Option<GCRA>>,
		get_master_league_limit: &'a Mutex<Option<GCRA>>,
		get_summoner_positions_limit: &'a Mutex<Option<GCRA>>,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			get_challenger_league_limit: get_challenger_league_limit,
			get_summoner_leagues_limit: get_summoner_leagues_limit,
			get_master_league_limit: get_master_league_limit,
			get_summoner_positions_limit: get_summoner_positions_limit,
		}
	}

	pub fn challengerleagues(&self) -> challengerleagues::Subclient<K> {
		challengerleagues::Subclient::new(
			self.region,
			self.key.clone(),
			&self.app_limit,
			&self.get_challenger_league_limit,
		)
	}

	pub fn leagues(&self) -> leagues::Subclient<K> {
		leagues::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.get_summoner_leagues_limit)
	}

	pub fn masterleagues(&self) -> masterleagues::Subclient<K> {
		masterleagues::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.get_master_league_limit)
	}

	pub fn positions(&self) -> positions::Subclient<K> {
		positions::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.get_summoner_positions_limit)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
