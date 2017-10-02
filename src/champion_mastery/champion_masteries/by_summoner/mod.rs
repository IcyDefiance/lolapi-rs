pub mod by_champion;
use {dto, request, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K> {
	region: &'static str,
	key: K,
	app_limit: &'a Mutex<Option<GCRA>>,
	get_champion_masteries_limit: &'a Mutex<Option<GCRA>>,
	get_champion_mastery_limit: &'a Mutex<Option<GCRA>>,
	summoner_id: i64,
}
impl<'a, K: Display + Clone> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: K,
		app_limit: &'a Mutex<Option<GCRA>>,
		get_champion_masteries_limit: &'a Mutex<Option<GCRA>>,
		get_champion_mastery_limit: &'a Mutex<Option<GCRA>>,
		summoner_id: i64,
	) -> Self {
		Self {
			region: region,
			key: key,
			app_limit: app_limit,
			get_champion_masteries_limit: get_champion_masteries_limit,
			get_champion_mastery_limit: get_champion_mastery_limit,
			summoner_id: summoner_id,
		}
	}

	/// "Get all champion mastery entries sorted by number of champion points descending."
	///
	/// **Endpoint**: `/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}`
	pub fn get(&self) -> Result<Vec<dto::ChampionMastery>, StatusCode> {
		let path = format!(
			"/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}",
			summoner_id = self.summoner_id
		);
		request(self.region, &self.key, &path, &self.app_limit, self.get_champion_masteries_limit)
	}

	pub fn by_champion(&self, champion_id: i64) -> by_champion::Subclient<K> {
		by_champion::Subclient::new(
			self.region,
			self.key.clone(),
			&self.app_limit,
			&self.get_champion_mastery_limit,
			self.summoner_id,
			champion_id,
		)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
