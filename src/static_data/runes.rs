use {dto, request_with_query, Locale, StaticDataRuneTags, StatusCode};
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

	/// "Retrieves rune list"
	///
	/// **Endpoint**: `/lol/static-data/v3/runes`
	pub fn get(
		&self,
		locale: Option<Locale>,
		version: Option<&str>,
		tags: &StaticDataRuneTags,
	) -> Result<dto::RuneList, StatusCode> {
		let path = "/lol/static-data/v3/runes";

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}
		let params = params.into_iter().chain(tags.to_query_pairs(&StaticDataRuneTags::none()).into_iter());

		request_with_query(self.region, self.key, path, params, None, &self.method_limits.get)
	}

	/// "Retrieves rune by ID"
	///
	/// **Endpoint**: `/lol/static-data/v3/runes/{id}`
	pub fn get_id(
		&self,
		id: i32,
		locale: Option<Locale>,
		version: Option<&str>,
		tags: &StaticDataRuneTags,
	) -> Result<dto::Rune, StatusCode> {
		let path = format!("/lol/static-data/v3/runes/{id}", id = id);

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}
		let params = params.into_iter().chain(tags.to_query_pairs(&StaticDataRuneTags::none()).into_iter());

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
			.runes()
			.get(Some(::Locale::en_US), None, &::StaticDataRuneTags { stats: true, ..::StaticDataRuneTags::none() })
			.unwrap();
	}

	#[test]
	fn get_id() {
		::CLIENT
			.static_data()
			.runes()
			.get_id(
				5001,
				Some(::Locale::en_US),
				None,
				&::StaticDataRuneTags { stats: true, ..::StaticDataRuneTags::none() },
			)
			.unwrap();
	}
}
