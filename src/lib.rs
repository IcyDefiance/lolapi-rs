extern crate hyper;
extern crate itertools;
extern crate num_rational;
extern crate ratelimit_meter;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

macro_rules! query_tags {
	($name:ident { $($field:ident: $string:expr),*, }) => { query_tags! { $name { $($field: $string),* } } };

	(
		$name:ident {
			$($field:ident: $string:expr),*
		}
	) => {
		#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
		pub struct $name {
			$(pub $field: bool),*
		}
		impl $name {
			pub fn all() -> Self {
				Self { $($field: true),* }
			}

			pub fn none() -> Self {
				Self { $($field: false),* }
			}

			pub(crate) fn to_query_pairs(&self, ignore: &$name) -> Vec<(&'static str, &'static str)> {
				// if all non-ignored fields are true
				if $((self.$field || ignore.$field))&&* {
					vec![("tags", "all")]
				} else {
					let mut ret = vec![];
					$(
						if self.$field && !ignore.$field {
							ret.push(("tags", $string))
						}
					)*
					ret
				}
			}
		}
	};
}

pub mod champion_mastery;
pub mod dto;
pub mod league;
pub mod platform;
pub mod static_data;

mod locale;
mod static_data_champion_tags;
mod static_data_item_tags;
mod static_data_mastery_tags;
mod queue_type;

pub use locale::Locale;
pub use queue_type::QueueType;
pub use static_data_champion_tags::StaticDataChampionTags;
pub use static_data_item_tags::StaticDataItemTags;
pub use static_data_mastery_tags::StaticDataMasteryTags;

use itertools::Itertools;
use num_rational::Ratio;
use ratelimit_meter::{Decider, Decision, GCRA};
use reqwest::{StatusCode, Url};
use reqwest::header::{Formatter, Header, Raw, RetryAfter};
use serde::de;
use std::borrow::Borrow;
use std::fmt::{self, Display};
use std::str;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Region {
	BR,
	EUNE,
	EUW,
	JP,
	KR,
	LAN,
	LAS,
	NA,
	OCE,
	TR,
	RU,
	PBE,
}
impl Region {
	fn to_str(self) -> &'static str {
		match self {
			Region::BR => "br1",
			Region::EUNE => "eun1",
			Region::EUW => "euw1",
			Region::JP => "jp1",
			Region::KR => "kr",
			Region::LAN => "la1",
			Region::LAS => "la2",
			Region::NA => "na1",
			Region::OCE => "oc1",
			Region::TR => "tr1",
			Region::RU => "ru",
			Region::PBE => "pbe1",
		}
	}
}

pub struct LolApiClient<K> {
	region: &'static str,
	key: K,
	app_limit: Mutex<Option<GCRA>>,
	champion_mastery_limits: champion_mastery::MethodLimits,
	league_limits: league::MethodLimits,
	platform_limits: platform::MethodLimits,
	static_data_limits: static_data::MethodLimits,
}
impl<K: Display + Clone> LolApiClient<K> {
	pub fn new(region: Region, key: K) -> Self {
		Self {
			region: region.to_str(),
			key: key,
			app_limit: Mutex::default(),
			champion_mastery_limits: champion_mastery::MethodLimits::new(),
			league_limits: league::MethodLimits::new(),
			platform_limits: platform::MethodLimits::new(),
			static_data_limits: static_data::MethodLimits::new(),
		}
	}

	pub fn champion_mastery(&self) -> champion_mastery::Subclient<K> {
		champion_mastery::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.champion_mastery_limits)
	}

	pub fn league(&self) -> league::Subclient<K> {
		league::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.league_limits)
	}

	pub fn platform(&self) -> platform::Subclient<K> {
		platform::Subclient::new(self.region, self.key.clone(), &self.app_limit, &self.platform_limits)
	}

	pub fn static_data(&self) -> static_data::Subclient<K> {
		static_data::Subclient::new(self.region, self.key.clone(), &self.static_data_limits)
	}
}
unsafe impl<K> Send for LolApiClient<K> {}
unsafe impl<K> Sync for LolApiClient<K> {}

fn request<T, Rk>(
	region: &str,
	key: Rk,
	route: &str,
	app_limit_mutex: Option<&Mutex<Option<GCRA>>>,
	method_limit_mutex: &Mutex<Option<GCRA>>,
) -> Result<T, StatusCode>
where
	T: de::DeserializeOwned,
	Rk: Display,
{
	request_with_query::<T, Rk, _, &str, &str>(region, key, route, &[], app_limit_mutex, method_limit_mutex)
}

fn request_with_query<T, Rk, I, K, V>(
	region: &str,
	key: Rk,
	route: &str,
	query: I,
	app_limit_mutex: Option<&Mutex<Option<GCRA>>>,
	method_limit_mutex: &Mutex<Option<GCRA>>,
) -> Result<T, StatusCode>
where
	T: de::DeserializeOwned,
	Rk: Display,
	I: IntoIterator,
	K: AsRef<str>,
	V: AsRef<str>,
	<I as IntoIterator>::Item: Borrow<(K, V)>,
{
	let url = Url::parse_with_params(
		&format!("https://{region}.api.riotgames.com{route}?api_key={key}", region = region, route = route, key = key),
		query,
	).unwrap();

	if let Some(app_limit_mutex) = app_limit_mutex {
		wait(&mut app_limit_mutex.lock().unwrap());
	}
	wait(&mut method_limit_mutex.lock().unwrap());

	loop {
		let mut response = reqwest::get(url.clone()).unwrap();

		match response.status() {
			StatusCode::TooManyRequests => {
				if let Some(app_limit_mutex) = app_limit_mutex {
					let app_limit = response.headers().get::<XAppRateLimit>();
					let app_limit_count = response.headers().get::<XAppRateLimitCount>();
					match (app_limit, app_limit_count) {
						(Some(&XAppRateLimit { ref limits }), Some(&XAppRateLimitCount { ref limit_counts })) => {
							*app_limit_mutex.lock().unwrap() = Some(headers_to_gcra(limits, limit_counts));
						},
						_ => (),
					}
				}

				let method_limit = response.headers().get::<XMethodRateLimit>();
				let method_limit_count = response.headers().get::<XMethodRateLimitCount>();
				match (method_limit, method_limit_count) {
					(Some(&XMethodRateLimit { ref limits }), Some(&XMethodRateLimitCount { ref limit_counts })) => {
						*method_limit_mutex.lock().unwrap() = Some(headers_to_gcra(&limits, &limit_counts));
					},
					_ => (),
				}

				match response.headers().get::<RetryAfter>() {
					Some(&RetryAfter::Delay(duration)) => thread::sleep(duration),
					Some(_) => unreachable!(),
					None => thread::sleep(Duration::from_secs(1)),
				}
			},
			StatusCode::Ok => return Ok(response.json().unwrap()),
			status => return Err(status),
		}
	}
}

fn wait(gcra: &mut Option<GCRA>) {
	if let Some(ref mut gcra) = *gcra {
		while let Decision::No(time) = gcra.check().unwrap() {
			thread::sleep(time.duration_since(Instant::now()));
		}
	}
}

fn headers_to_gcra(limits: &[(u64, std::time::Duration)], limit_counts: &[(u64, std::time::Duration)]) -> GCRA {
	let rate = limits
		.iter()
		.zip(limit_counts.into_iter())
		.map(|(l, lc)| {
			assert!(l.1 == lc.1);
			Ratio::new(l.0, l.1.as_secs())
		})
		.min()
		.unwrap();

	GCRA::for_capacity((*rate.numer()) as u32).unwrap().per(Duration::from_secs(*rate.denom())).build()
}

#[derive(Clone)]
struct XAppRateLimit {
	limits: Vec<(u64, Duration)>,
}
impl Header for XAppRateLimit {
	fn header_name() -> &'static str {
		"X-App-Rate-Limit"
	}

	fn parse_header(raw: &Raw) -> Result<Self, hyper::Error> {
		let limits = str::from_utf8(raw.one().unwrap())
			.unwrap()
			.split(',')
			.map(|limit| {
				let mut nums = limit.split(':').map(|x| x.parse::<u64>().unwrap());
				(nums.next().unwrap(), Duration::from_secs(nums.next().unwrap()))
			})
			.collect();

		Ok(Self { limits: limits })
	}

	fn fmt_header(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		f.fmt_line(&self.limits.iter().map(|&(count, duration)| format!("{}:{}", count, duration.as_secs())).join(","))
	}
}

#[derive(Clone)]
struct XAppRateLimitCount {
	limit_counts: Vec<(u64, Duration)>,
}
impl Header for XAppRateLimitCount {
	fn header_name() -> &'static str {
		"X-App-Rate-Limit-Count"
	}

	fn parse_header(raw: &Raw) -> Result<Self, hyper::Error> {
		let limit_counts = str::from_utf8(raw.one().unwrap())
			.unwrap()
			.split(',')
			.map(|limit| {
				let mut nums = limit.split(':').map(|x| x.parse::<u64>().unwrap());
				(nums.next().unwrap(), Duration::from_secs(nums.next().unwrap()))
			})
			.collect();

		Ok(Self { limit_counts: limit_counts })
	}

	fn fmt_header(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		f.fmt_line(
			&self.limit_counts.iter().map(|&(count, duration)| format!("{}:{}", count, duration.as_secs())).join(","),
		)
	}
}

#[derive(Clone)]
struct XMethodRateLimit {
	limits: Vec<(u64, Duration)>,
}
impl Header for XMethodRateLimit {
	fn header_name() -> &'static str {
		"X-Method-Rate-Limit"
	}

	fn parse_header(raw: &Raw) -> Result<Self, hyper::Error> {
		let limits = str::from_utf8(raw.one().unwrap())
			.unwrap()
			.split(',')
			.map(|limit| {
				let mut nums = limit.split(':').map(|x| x.parse::<u64>().unwrap());
				(nums.next().unwrap(), Duration::from_secs(nums.next().unwrap()))
			})
			.collect();

		Ok(Self { limits: limits })
	}

	fn fmt_header(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		f.fmt_line(&self.limits.iter().map(|&(count, duration)| format!("{}:{}", count, duration.as_secs())).join(","))
	}
}

#[derive(Clone)]
struct XMethodRateLimitCount {
	limit_counts: Vec<(u64, Duration)>,
}
impl Header for XMethodRateLimitCount {
	fn header_name() -> &'static str {
		"X-Method-Rate-Limit-Count"
	}

	fn parse_header(raw: &Raw) -> Result<Self, hyper::Error> {
		let limit_counts = str::from_utf8(raw.one().unwrap())
			.unwrap()
			.split(',')
			.map(|limit| {
				let mut nums = limit.split(':').map(|x| x.parse::<u64>().unwrap());
				(nums.next().unwrap(), Duration::from_secs(nums.next().unwrap()))
			})
			.collect();

		Ok(Self { limit_counts: limit_counts })
	}

	fn fmt_header(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		f.fmt_line(
			&self.limit_counts.iter().map(|&(count, duration)| format!("{}:{}", count, duration.as_secs())).join(","),
		)
	}
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
lazy_static! {
	pub static ref CLIENT: ::LolApiClient<&'static str> = ::LolApiClient::new(::Region::NA, env!("LOL_API_KEY"));
}
