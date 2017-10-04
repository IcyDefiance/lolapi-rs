pub mod champions;
pub mod items;
pub mod language_strings;
pub mod languages;
pub mod maps;
pub mod masteries;
pub mod profile_icons;
use std::fmt::Display;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	method_limits: &'a MethodLimits,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(region: &'static str, key: &'a K, method_limits: &'a MethodLimits) -> Self {
		Self { region: region, key: key, method_limits: method_limits }
	}

	pub fn champions(&self) -> champions::Subclient<K> {
		champions::Subclient::new(self.region, self.key, &self.method_limits.champions)
	}

	pub fn items(&self) -> items::Subclient<K> {
		items::Subclient::new(self.region, self.key, &self.method_limits.items)
	}

	pub fn language_strings(&self) -> language_strings::Subclient<K> {
		language_strings::Subclient::new(self.region, self.key, &self.method_limits.language_strings)
	}

	pub fn languages(&self) -> languages::Subclient<K> {
		languages::Subclient::new(self.region, self.key, &self.method_limits.languages)
	}

	pub fn maps(&self) -> maps::Subclient<K> {
		maps::Subclient::new(self.region, self.key, &self.method_limits.maps)
	}

	pub fn masteries(&self) -> masteries::Subclient<K> {
		masteries::Subclient::new(self.region, self.key, &self.method_limits.masteries)
	}

	pub fn profile_icons(&self) -> profile_icons::Subclient<K> {
		profile_icons::Subclient::new(self.region, self.key, &self.method_limits.profile_icons)
	}
}
unsafe impl<'a, K> Send for Subclient<'a, K> {}
unsafe impl<'a, K> Sync for Subclient<'a, K> {}

pub(super) struct MethodLimits {
	champions: champions::MethodLimits,
	items: items::MethodLimits,
	language_strings: language_strings::MethodLimits,
	languages: languages::MethodLimits,
	maps: maps::MethodLimits,
	masteries: masteries::MethodLimits,
	profile_icons: profile_icons::MethodLimits,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self {
			champions: champions::MethodLimits::new(),
			items: items::MethodLimits::new(),
			language_strings: language_strings::MethodLimits::new(),
			languages: languages::MethodLimits::new(),
			maps: maps::MethodLimits::new(),
			masteries: masteries::MethodLimits::new(),
			profile_icons: profile_icons::MethodLimits::new(),
		}
	}
}
