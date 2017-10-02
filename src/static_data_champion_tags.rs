#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StaticDataChampionTags {
	pub allytips: bool,
	pub blurb: bool,
	pub enemytips: bool,
	pub format: bool,
	pub image: bool,
	pub info: bool,
	pub keys: bool,
	pub lore: bool,
	pub partype: bool,
	pub passive: bool,
	pub recommended: bool,
	pub skins: bool,
	pub spells: bool,
	pub stats: bool,
	pub tags: bool,
}
impl StaticDataChampionTags {
	pub fn all() -> Self {
		Self {
			allytips: true,
			blurb: true,
			enemytips: true,
			format: true,
			image: true,
			info: true,
			keys: true,
			lore: true,
			partype: true,
			passive: true,
			recommended: true,
			skins: true,
			spells: true,
			stats: true,
			tags: true,
		}
	}

	pub fn none() -> Self {
		Self {
			allytips: false,
			blurb: false,
			enemytips: false,
			format: false,
			image: false,
			info: false,
			keys: false,
			lore: false,
			partype: false,
			passive: false,
			recommended: false,
			skins: false,
			spells: false,
			stats: false,
			tags: false,
		}
	}

	pub(super) fn to_query_pairs(&self) -> Vec<(&'static str, &'static str)> {
		if self == &StaticDataChampionTags::all() {
			vec![("tags", "all")]
		} else {
			let mut ret = vec![];
			if self.allytips {
				ret.push(("tags", "allytips"));
			}
			if self.blurb {
				ret.push(("tags", "blurb"));
			}
			if self.enemytips {
				ret.push(("tags", "enemytips"));
			}
			if self.format {
				ret.push(("tags", "format"));
			}
			if self.image {
				ret.push(("tags", "image"));
			}
			if self.info {
				ret.push(("tags", "info"));
			}
			if self.keys {
				ret.push(("tags", "keys"));
			}
			if self.lore {
				ret.push(("tags", "lore"));
			}
			if self.partype {
				ret.push(("tags", "partype"));
			}
			if self.passive {
				ret.push(("tags", "passive"));
			}
			if self.recommended {
				ret.push(("tags", "recommended"));
			}
			if self.skins {
				ret.push(("tags", "skins"));
			}
			if self.spells {
				ret.push(("tags", "spells"));
			}
			if self.stats {
				ret.push(("tags", "stats"));
			}
			if self.tags {
				ret.push(("tags", "tags"));
			}
			ret
		}
	}
}
