use {dto, request_with_query, Locale, MasteryTags, StatusCode};
use ratelimit_meter::LeakyBucket;
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

	/// "Retrieves mastery list"
	///
	/// **Endpoint**: `/lol/static-data/v3/masteries`
	pub fn get(
		&mut self,
		locale: Option<Locale>,
		version: Option<&str>,
		tags: &MasteryTags,
	) -> Result<dto::MasteryList, StatusCode> {
		let path = "/lol/static-data/v3/masteries";

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}
		let params = params.into_iter().chain(tags.to_query_pairs(&MasteryTags::none()).into_iter());

		request_with_query(self.region, self.key, path, params, &mut vec![], &mut self.method_limits.get)
	}

	/// "Retrieves mastery item by ID"
	///
	/// `tags.tree` is ignored.
	///
	/// **Endpoint**: `/lol/static-data/v3/masteries/{id}`
	pub fn get_id(
		&mut self,
		id: i32,
		locale: Option<Locale>,
		version: Option<&str>,
		tags: &MasteryTags,
	) -> Result<dto::Mastery, StatusCode> {
		let mut tags = *tags;
		tags.tree = false;

		let path = format!("/lol/static-data/v3/masteries/{id}", id = id);

		let mut params = vec![];
		if let Some(locale) = locale {
			params.push(("locale", locale.to_str()));
		}
		if let Some(version) = version {
			params.push(("version", version));
		}
		let params = params
			.into_iter()
			.chain(tags.to_query_pairs(&MasteryTags { tree: true, ..MasteryTags::none() }).into_iter());

		request_with_query(self.region, self.key, &path, params, &mut vec![], &mut self.method_limits.get_id)
	}
}

pub(super) struct MethodLimits {
	get: Vec<LeakyBucket>,
	get_id: Vec<LeakyBucket>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: vec![], get_id: vec![] }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT.lock().unwrap()
			.static_data_v3()
			.masteries()
			.get(Some(::Locale::en_US), None, &::MasteryTags { image: true, ..::MasteryTags::none() })
			.unwrap();
	}

	#[test]
	fn get_id() {
		::CLIENT.lock().unwrap()
			.static_data_v3()
			.masteries()
			.get_id(6111, Some(::Locale::en_US), None, &::MasteryTags { image: true, ..::MasteryTags::none() })
			.unwrap();
	}
}
