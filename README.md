# lolapi
Rate limited League of Legends API wrapper for Rust (WIP)

---

### Important
Rate limiting is implemented within `LolClient`, so you should only create one client per region, and you should
reuse it instead of creating a new one for each request.

---

### Links
* [Crate](https://crates.io/crates/lolapi)
* [Documentation](https://docs.rs/lolapi)

### Currently supported
- [ ] CHAMPION-MASTERY-V3
- [ ] CHAMPION-V3
- [ ] LEAGUE-V3
- [ ] LOL-STATIC-DATA-V3
- [ ] LOL-STATUS-V3
- [ ] MATCH-V3 (partial support)
- [ ] SPECTATOR-V3
- [ ] SUMMONER-V3
- [ ] TOURNAMENT-STUB-V3
- [ ] TOURNAMENT-V3
