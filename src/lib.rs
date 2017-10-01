extern crate hyper;
extern crate itertools;
extern crate num_rational;
extern crate ratelimit_meter;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
pub mod dto;
use itertools::Itertools;
use num_rational::Ratio;
use ratelimit_meter::{Decider, Decision, GCRA};
use reqwest::StatusCode;
use reqwest::header::{Formatter, Header, Raw, RetryAfter};
use serde::de::DeserializeOwned;
use std::fmt::{self, Display};
use std::str;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

pub enum Region {
	NA1,
}

pub struct LolApiClient<K> {
	region: &'static str,
	key: K,
	app_limit: Mutex<Option<GCRA>>,
	get_champion_masteries_limit: Mutex<Option<GCRA>>,
	get_champions_limit: Mutex<Option<GCRA>>,
	get_champion_limit: Mutex<Option<GCRA>>,
}
impl<K: Display> LolApiClient<K> {
	pub fn new(region: Region, key: K) -> Self {
		Self {
			region: region_to_str(region),
			key: key,
			app_limit: Mutex::default(),
			get_champion_masteries_limit: Mutex::default(),
			get_champions_limit: Mutex::default(),
			get_champion_limit: Mutex::default(),
		}
	}

	pub fn get_champion_masteries(&self, summoner_id: u64) -> Result<Vec<dto::ChampionMastery>, StatusCode> {
		self.request(
			&format!(
				"/lol/champion-mastery/v3/champion-masteries/by-summoner/{summoner_id}",
				summoner_id = summoner_id
			),
			|| self.get_champion_masteries_limit.lock().unwrap(),
		)
	}

	pub fn get_champions(&self) -> Result<Vec<dto::Champion>, StatusCode> {
		self.request::<dto::ChampionList, _>("/lol/platform/v3/champions", || self.get_champions_limit.lock().unwrap())
			.map(|x| x.champions)
	}

	pub fn get_champion(&self, id: u64) -> Result<dto::Champion, StatusCode> {
		self.request(&format!("/lol/platform/v3/champions/{id}", id = id), || self.get_champion_limit.lock().unwrap())
	}

	fn request<'a, T, F>(&self, route: &str, mut method_limit_lock: F) -> Result<T, StatusCode>
	where
		T: DeserializeOwned,
		F: FnMut() -> MutexGuard<'a, Option<GCRA>>,
	{
		wait(&mut self.app_limit.lock().unwrap());
		wait(&mut method_limit_lock());

		loop {
			let mut response = reqwest::get(&format!(
				"https://{region}.api.riotgames.com{route}?api_key={key}",
				region = self.region,
				route = route,
				key = self.key
			)).unwrap();

			match response.status() {
				StatusCode::TooManyRequests => {
					let mut app_limit_lock = self.app_limit.lock().unwrap();
					let mut method_limit_lock = method_limit_lock();

					let app_limit = response.headers().get::<XAppRateLimit>();
					let app_limit_count = response.headers().get::<XAppRateLimitCount>();
					match (app_limit, app_limit_count) {
						(Some(&XAppRateLimit { ref limits }), Some(&XAppRateLimitCount { ref limit_counts })) => {
							*app_limit_lock = Some(headers_to_gcra(limits, limit_counts));
						},
						_ => (),
					}

					let method_limit = response.headers().get::<XMethodRateLimit>();
					let method_limit_count = response.headers().get::<XMethodRateLimitCount>();
					match (method_limit, method_limit_count) {
						(Some(&XMethodRateLimit { ref limits }), Some(&XMethodRateLimitCount { ref limit_counts })) => {
							*method_limit_lock = Some(headers_to_gcra(&limits, &limit_counts));
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
}

fn region_to_str(region: Region) -> &'static str {
	match region {
		Region::NA1 => "na1",
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
mod tests {
	lazy_static! {
		static ref CLIENT: ::LolApiClient<&'static str> = ::LolApiClient::new(::Region::NA1, env!("LOL_API_KEY"));
	}

	// #[test]
	// fn get_champion_masteries() {
	// 	CLIENT.get_champion_masteries().unwrap();
	// }

	#[test]
	fn get_champions() {
		CLIENT.get_champions().unwrap();
	}

	#[test]
	fn get_champion() {
		CLIENT.get_champion(266).unwrap();
	}
}
