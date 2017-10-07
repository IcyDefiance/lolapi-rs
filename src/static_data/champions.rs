use {dto, request_with_query, ChampionTags, Locale, StatusCode};
use ratelimit_meter::GCRA;
use std::fmt::Display;
use std::sync::Mutex;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: &'a K, method_limits: &'a MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	/// "Retrieves champion list"
	///
	/// **Endpoint**: `/lol/static-data/v3/champions`
	pub fn get(
		&self,
		locale: Option<Locale>,
		version: Option<&str>,
		tags: &ChampionTags,
	) -> Result<dto::ChampionListStatic, StatusCode> {
		let path = "/lol/static-data/v3/champions";

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}
		let params = params.into_iter().chain(tags.to_query_pairs(&ChampionTags::none()).into_iter());

		request_with_query(self.region, self.key, path, params, None, &self.method_limits.get)
	}

	/// "Retrieves champion by ID"
	///
	/// `tags.format` and `tags.keys` are ignored.
	///
	/// **Endpoint**: `/lol/static-data/v3/champions/{id}`
	pub fn get_id(
		&self,
		id: i32,
		locale: Option<Locale>,
		version: Option<&str>,
		tags: &ChampionTags,
	) -> Result<dto::ChampionStatic, StatusCode> {
		let path = format!("/lol/static-data/v3/champions/{id}", id = id);

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}
		let params = params
			.into_iter()
			.chain(tags.to_query_pairs(&ChampionTags { format: true, keys: true, ..ChampionTags::none() }).into_iter());

		request_with_query(self.region, self.key, &path, params, None, &self.method_limits.get_id)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	get: Mutex<Option<GCRA>>,
	get_id: Mutex<Option<GCRA>>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: Mutex::default(), get_id: Mutex::default() }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT
			.static_data()
			.champions()
			.get(
				Some(::Locale::en_US),
				None,
				&::ChampionTags { allytips: true, enemytips: true, ..::ChampionTags::none() },
			)
			.unwrap();
	}

	#[test]
	fn get_id() {
		::CLIENT
			.static_data()
			.champions()
			.get_id(
				266,
				Some(::Locale::en_US),
				None,
				&::ChampionTags { allytips: true, enemytips: true, ..::ChampionTags::none() },
			)
			.unwrap();
	}
}
