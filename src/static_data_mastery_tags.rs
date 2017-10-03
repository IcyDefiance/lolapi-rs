#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StaticDataMasteryTags {
	pub image: bool,
	pub mastery_tree: bool,
	pub prereq: bool,
	pub ranks: bool,
	pub sanitized_description: bool,
	pub tree: bool,
}
impl StaticDataMasteryTags {
	pub fn all() -> Self {
		Self {
			image: true,
			mastery_tree: true,
			prereq: true,
			ranks: true,
			sanitized_description: true,
			tree: true,
		}
	}

	pub fn none() -> Self {
		Self {
			image: false,
			mastery_tree: false,
			prereq: false,
			ranks: false,
			sanitized_description: false,
			tree: false,
		}
	}

	pub(super) fn to_query_pairs(&self) -> Vec<(&'static str, &'static str)> {
		if self == &StaticDataMasteryTags::all() {
			vec![("tags", "all")]
		} else {
			let mut ret = vec![];
			if self.image {
				ret.push(("tags", "image"));
			}
			if self.mastery_tree {
				ret.push(("tags", "masteryTree"));
			}
			if self.prereq {
				ret.push(("tags", "prereq"));
			}
			if self.ranks {
				ret.push(("tags", "ranks"));
			}
			if self.sanitized_description {
				ret.push(("tags", "sanitizedDescription"));
			}
			if self.tree {
				ret.push(("tags", "tree"));
			}
			ret
		}
	}
}
