use {dto, request, request_with_query, StatusCode};
use ratelimit_meter::LeakyBucket;
use std::fmt::Display;
use time::Timespec;

pub struct Subclient<'a, K: 'a> {
	region: &'static str,
	key: &'a K,
	app_limits: &'a mut Vec<LeakyBucket>,
	method_limits: &'a mut MethodLimits,
	account_id: i64,
}
impl<'a, K: Display> Subclient<'a, K> {
	pub(super) fn new(
		region: &'static str,
		key: &'a K,
		app_limits: &'a mut Vec<LeakyBucket>,
		method_limits: &'a mut MethodLimits,
		account_id: i64,
	) -> Self {
		Self { region: region, key: key, app_limits: app_limits, method_limits: method_limits, account_id: account_id }
	}

	/// "Get matchlist for games played on given account ID and platform ID and filtered using given filter parameters,
	/// if any."
	///
	/// **Endpoint**: `/lol/match/v3/matchlists/by-account/{account_id}`
	pub fn get<Q, S, C>(
		&mut self,
		begin_index: Option<i32>,
		end_index: Option<i32>,
		begin_time: Option<Timespec>,
		end_time: Option<Timespec>,
		queues: Q,
		seasons: S,
		champions: C,
	) -> Result<dto::Matchlist, StatusCode>
	where
		Q: Iterator<Item = i32>,
		S: Iterator<Item = i32>,
		C: Iterator<Item = i32>,
	{
		let path = format!("/lol/match/v3/matchlists/by-account/{account_id}", account_id = self.account_id);

		let mut params = vec![];
		if let Some(begin_index) = begin_index {
			params.push(("beginIndex", begin_index.to_string()));
		}
		if let Some(end_index) = end_index {
			params.push(("endIndex", end_index.to_string()));
		}
		if let Some(begin_time) = begin_time {
			params.push(("beginTime", (begin_time.sec * 1000 + begin_time.nsec as i64 / 1_000_000).to_string()));
		}
		if let Some(end_time) = end_time {
			params.push(("endTime", (end_time.sec * 1000 + end_time.nsec as i64 / 1_000_000).to_string()));
		}
		params.extend(queues.map(|queue| ("queue", queue.to_string())));
		params.extend(seasons.map(|season| ("season", season.to_string())));
		params.extend(champions.map(|champion| ("champion", champion.to_string())));

		request_with_query(self.region, self.key, &path, params, self.app_limits, &mut self.method_limits.get)
	}

	/// "Get matchlist for last 20 matches played on given account ID and platform ID."
	///
	/// **Endpoint**: `/lol/match/v3/matchlists/by-account/{account_id}/recent`
	pub fn recent(&mut self) -> Result<dto::Matchlist, StatusCode>
	{
		let path = format!("/lol/match/v3/matchlists/by-account/{account_id}/recent", account_id = self.account_id);
		request(self.region, self.key, &path, self.app_limits, &mut self.method_limits.get)
	}
}

pub(super) struct MethodLimits {
	get: Vec<LeakyBucket>,
}
impl MethodLimits {
	pub fn new() -> Self {
		Self { get: vec![] }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn get() {
		::CLIENT
			.lock()
			.unwrap()
			.match_v3()
			.matchlists()
			.by_account(38559235)
			.get(None, None, None, None, vec![].into_iter(), vec![].into_iter(), vec![].into_iter())
			.unwrap();
	}

	#[test]
	fn recent() {
		::CLIENT
			.lock()
			.unwrap()
			.match_v3()
			.matchlists()
			.by_account(38559235)
			.recent()
			.unwrap();
	}
}
