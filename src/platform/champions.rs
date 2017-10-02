use {dto, request, StatusCode};
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
impl<'a, K: Display> Subclient<'a, K> {
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

	/// "Retrieve all champions."
	///
	/// **Endpoint**: `/lol/platform/v3/champions`
	pub fn get(&self) -> Result<Vec<dto::Champion>, StatusCode> {
		let path = "/lol/platform/v3/champions";
		request::<dto::ChampionList, _>(self.region, &self.key, path, &self.app_limit, self.get_champions_limit)
			.map(|x| x.champions)
	}

	/// "Retrieve champion by ID."
	///
	/// **Endpoint**: `/lol/platform/v3/champions/{id}`
	pub fn get_id(&self, id: i64) -> Result<dto::Champion, StatusCode> {
		let path = format!("/lol/platform/v3/champions/{id}", id = id);
		request(self.region, &self.key, &path, &self.app_limit, self.get_champion_limit)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}
