#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StaticDataItemTags {
	pub colloq: bool,
	pub consume_on_full: bool,
	pub consumed: bool,
	pub depth: bool,
	pub effect: bool,
	pub from: bool,
	pub gold: bool,
	pub groups: bool,
	pub hide_from_all: bool,
	pub image: bool,
	pub in_store: bool,
	pub into: bool,
	pub maps: bool,
	pub required_champion: bool,
	pub sanitized_description: bool,
	pub special_recipe: bool,
	pub stacks: bool,
	pub stats: bool,
	pub tags: bool,
	pub tree: bool,
}
impl StaticDataItemTags {
	pub fn all() -> Self {
		Self {
			colloq: true,
			consume_on_full: true,
			consumed: true,
			depth: true,
			effect: true,
			from: true,
			gold: true,
			groups: true,
			hide_from_all: true,
			image: true,
			in_store: true,
			into: true,
			maps: true,
			required_champion: true,
			sanitized_description: true,
			special_recipe: true,
			stacks: true,
			stats: true,
			tags: true,
			tree: true,
		}
	}

	pub fn none() -> Self {
		Self {
			colloq: false,
			consume_on_full: false,
			consumed: false,
			depth: false,
			effect: false,
			from: false,
			gold: false,
			groups: false,
			hide_from_all: false,
			image: false,
			in_store: false,
			into: false,
			maps: false,
			required_champion: false,
			sanitized_description: false,
			special_recipe: false,
			stacks: false,
			stats: false,
			tags: false,
			tree: false,
		}
	}

	pub(super) fn to_query_pairs(&self) -> Vec<(&'static str, &'static str)> {
		if self == &Self::all() {
			vec![("tags", "all")]
		} else {
			let mut ret = vec![];
			if self.colloq {
				ret.push(("tags", "colloq"));
			}
			if self.consume_on_full {
				ret.push(("tags", "consume_on_full"));
			}
			if self.consumed {
				ret.push(("tags", "consumed"));
			}
			if self.depth {
				ret.push(("tags", "depth"));
			}
			if self.effect {
				ret.push(("tags", "effect"));
			}
			if self.from {
				ret.push(("tags", "from"));
			}
			if self.gold {
				ret.push(("tags", "gold"));
			}
			if self.groups {
				ret.push(("tags", "groups"));
			}
			if self.hide_from_all {
				ret.push(("tags", "hide_from_all"));
			}
			if self.image {
				ret.push(("tags", "image"));
			}
			if self.in_store {
				ret.push(("tags", "in_store"));
			}
			if self.into {
				ret.push(("tags", "into"));
			}
			if self.maps {
				ret.push(("tags", "maps"));
			}
			if self.required_champion {
				ret.push(("tags", "required_champion"));
			}
			if self.sanitized_description {
				ret.push(("tags", "sanitized_description"));
			}
			if self.special_recipe {
				ret.push(("tags", "special_recipe"));
			}
			if self.stacks {
				ret.push(("tags", "stacks"));
			}
			if self.stats {
				ret.push(("tags", "stats"));
			}
			if self.tags {
				ret.push(("tags", "tags"));
			}
			if self.tree {
				ret.push(("tags", "tree"));
			}
			ret
		}
	}
}
