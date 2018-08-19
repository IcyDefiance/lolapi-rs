extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate ratelimit_meter;
#[macro_use]
extern crate rest_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate tokio_timer;

mod dto;

use futures::{ Future, future::{ loop_fn, ok, Loop } };
use std::{
	cmp::max,
	collections::HashMap,
	fmt,
	time::{ Duration, Instant },
	sync::{ Arc, Mutex }
};
use hyper::{ Body, Response, StatusCode };
use ratelimit_meter::{ Decider, LeakyBucket, MultiDecider };
use rest_client::{ AfterRequestResponse, RequestError, RequestHooks };

rest_client! {
	LolClient("https://{}.api.riotgames.com/lol/", platform: Platform)(LolClientHooks, ::tokio_timer::Error) {
		match_v3("match/v3/") {
			{},
			{
				matches("matches/") {
					{},
					{
						id("{}", matchid: i64) { { get -> ::dto::Match }, {} }
					}
				}

			}
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Platform { BR1, EUN1, EUW1, JP1, KR, LA1, LA2, NA, NA1, OC1, TR1, RU, PBE1 }
impl fmt::Display for Platform {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Platform::BR1 => write!(f, "br1"),
			Platform::EUN1 => write!(f, "eun1"),
			Platform::EUW1 => write!(f, "euw1"),
			Platform::JP1 => write!(f, "jp1"),
			Platform::KR => write!(f, "ke"),
			Platform::LA1 => write!(f, "la1"),
			Platform::LA2 => write!(f, "la2"),
			Platform::NA => write!(f, "na"),
			Platform::NA1 => write!(f, "na1"),
			Platform::OC1 => write!(f, "oc1"),
			Platform::TR1 => write!(f, "tr1"),
			Platform::RU => write!(f, "ru"),
			Platform::PBE1 => write!(f, "pbe1"),
		}
	}
}

pub struct LolClientHooks {
	inner: Arc<Mutex<LolClientHooksData>>,
}
impl LolClientHooks {
	pub fn new(app_limits: Vec<(u32, u64)>) -> Self {
		Self {
			inner:
				Arc::new(Mutex::new(LolClientHooksData {
					app_limits: app_limits.iter()
						.map(|&(limit, secs)| LeakyBucket::new(limit, Duration::from_secs(secs)).unwrap())
						.collect(),
					app_limit_vals: app_limits,
					method_limits: HashMap::new(),
					app_retry_after: Instant::now(),
					method_retry_afters: HashMap::new(),
					next_service_retry_afters: HashMap::new(),
				}))
		}
	}
}
impl RequestHooks<tokio_timer::Error> for LolClientHooks {
	fn before_request(&self, path: &str) -> Box<Future<Item = (), Error = tokio_timer::Error>> {
		let path = path.to_owned();

		Box::new(loop_fn(
			(self.inner.clone(), 0),
			move |(inner, i)| -> Box<Future<Item = _, Error = _>> {
				let inner2 = inner.clone();
				let mut inner_lock = inner2.lock().unwrap();

				let now = Instant::now();
				if inner_lock.app_retry_after > now {
					return Box::new(tokio_timer::sleep(inner_lock.app_retry_after - now).map(move |_| Loop::Continue((inner, i))));
				} else if let Some(&method_retry_after) = inner_lock.method_retry_afters.get(&path) {
					if method_retry_after > now {
						return Box::new(tokio_timer::sleep(method_retry_after - now).map(move |_| Loop::Continue((inner, i))));
					}
				}

				for (i, limit) in inner_lock.app_limits.iter_mut().skip(i).enumerate() {
					if let Err(err) = limit.check() {
						return Box::new(tokio_timer::sleep(err.wait_time()).map(move |_| Loop::Continue((inner, i))));
					}
				}

				Box::new(ok(Loop::Break(())))
			}
		))
	}

	fn after_request(
		&self,
		path: &str,
		future: impl Future<Item = Response<Body>, Error = RequestError<tokio_timer::Error>> + 'static
	) -> Box<Future<Item = AfterRequestResponse, Error = RequestError<tokio_timer::Error>>> {
		let inner = self.inner.clone();
		let path = path.to_owned();

		Box::new(future.map(move |res| {
			let mut inner_lock = inner.lock().unwrap();

			if let Some(app_rate_limit) = res.headers().get("X-App-Rate-Limit").map(|x| x.to_str().unwrap()) {
				let app_limit_vals: Vec<(u32, u64)> = app_rate_limit.split(',')
					.map(|x| x.split(':').collect::<Vec<_>>())
					.map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap()))
					.collect();

				if app_limit_vals != inner_lock.app_limit_vals {
					inner_lock.app_limits = app_limit_vals.iter()
						.map(|&(limit, secs)| LeakyBucket::new(limit, Duration::from_secs(secs)).unwrap())
						.collect();
					inner_lock.app_limit_vals = app_limit_vals;

					let app_rate_limit_count = res.headers().get("X-App-Rate-Limit-Count").unwrap().to_str().unwrap();
					for (i, leak) in app_rate_limit_count.split(',')
						.map(|x| x.split(':').nth(0).unwrap().parse::<u32>().unwrap())
						.enumerate()
					{
						inner_lock.app_limits[i].check_n(leak).unwrap();
					}
				}
			}

			if let Some(rate_limit) = res.headers().get("X-Method-Rate-Limit").map(|x| x.to_str().unwrap()) {
				inner_lock.method_limits.entry(path.clone())
					.and_modify(|(method_limits, method_limit_vals)| {
						let limit_vals: Vec<(u32, u64)> = rate_limit.split(',')
							.map(|x| x.split(':').collect::<Vec<_>>())
							.map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap()))
							.collect();

						if limit_vals == *method_limit_vals {
							return;
						}

						*method_limits = limit_vals.iter()
							.map(|&(limit, secs)| LeakyBucket::new(limit, Duration::from_secs(secs)).unwrap())
							.collect();
						*method_limit_vals = limit_vals;

						let rate_limit_count = res.headers().get("X-Method-Rate-Limit-Count").unwrap().to_str().unwrap();
						for (i, leak) in rate_limit_count.split(',')
							.map(|x| x.split(':').nth(0).unwrap().parse::<u32>().unwrap())
							.enumerate()
						{
							method_limits[i].check_n(leak).unwrap();
						}
					})
					.or_insert_with(|| {
						let limit_vals: Vec<(u32, u64)> = rate_limit.split(',')
							.map(|x| x.split(':').collect::<Vec<_>>())
							.map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap()))
							.collect();

						(
							limit_vals.iter()
								.map(|&(limit, secs)| LeakyBucket::new(limit, Duration::from_secs(secs)).unwrap())
								.collect(),
							limit_vals
						)
					});
			}

			if res.status() == StatusCode::TOO_MANY_REQUESTS {
				if let Some(rate_limit_type) = res.headers().get("X-Rate-Limit-Type").map(|x| x.to_str().unwrap()) {
					let retry_after = res.headers().get("Retry-After").unwrap().to_str().unwrap().parse::<u64>().unwrap();

					if rate_limit_type == "application" {
						inner_lock.app_retry_after = Instant::now() + Duration::from_secs(retry_after);
					} else if rate_limit_type == "method" {
						inner_lock.method_retry_afters.insert(path, Instant::now() + Duration::from_secs(retry_after));
					} else if rate_limit_type == "service" {
						inner_lock.method_retry_afters.insert(path, Instant::now() + Duration::from_secs(retry_after));
					} else {
						panic!("Invalid X-Rate-Limit-Type");
					}
				} else {
					let retry_after = inner_lock.next_service_retry_afters.get(&path).map(|&x| x).unwrap_or(1);
					inner_lock.method_retry_afters.insert(path.clone(), Instant::now() + Duration::from_secs(retry_after));
					inner_lock.next_service_retry_afters.insert(path, max(retry_after * 2, 64));
				}

				AfterRequestResponse::Retry
			} else {
				inner_lock.next_service_retry_afters.insert(path, 1);
				AfterRequestResponse::Continue(res)
			}
		}))
	}
}

struct LolClientHooksData {
	app_limits: Vec<LeakyBucket>,
	app_limit_vals: Vec<(u32, u64)>,
	method_limits: HashMap<String, (Vec<LeakyBucket>, Vec<(u32, u64)>)>,
	app_retry_after: Instant,
	method_retry_afters: HashMap<String, Instant>,
	next_service_retry_afters: HashMap<String, u64>,
}
