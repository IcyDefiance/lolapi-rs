# lolapi

---

### Important

Rate limiting is implemented within `LolApiClient`, so you should only create one client per region, and you should reuse it instead of creating a new one for each request. Personally, I'd recommend making it global with [lazy_static](https://crates.io/crates/lazy_static).

---

### Links

* [Crate](https://crates.io/crates/lolapi)
* [Documentation](https://docs.rs/lolapi/0.1.0/)

### Currently supports

- [x] CHAMPION-MASTERY-V3
- [x] CHAMPION-V3
- [x] LEAGUE-V3
- [ ] LOL-STATIC-DATA-V3
- [ ] LOL-STATUS-V3
- [ ] MASTERIES-V3
- [ ] MATCH-V3
- [ ] RUNES-V3
- [ ] SPECTATOR-V3
- [ ] SUMMONER-V3
- [ ] TOURNAMENT-STUB-V3
- [ ] TOURNAMENT-V3
