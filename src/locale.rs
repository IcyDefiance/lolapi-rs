#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Locale {
	cs_CZ,
	de_DE,
	el_GR,
	en_AU,
	en_GB,
	en_PH,
	en_PL,
	en_SG,
	en_US,
	es_AR,
	es_ES,
	es_MX,
	fr_FR,
	hu_HU,
	id_ID,
	it_IT,
	ja_JP,
	ko_KR,
	ms_MY,
	pl_PL,
	pt_BR,
	ro_RO,
	ru_RU,
	th_TH,
	tr_TR,
	vn_VN,
	zh_CN,
	zh_MY,
	zh_TW,
}
impl Locale {
	pub(super) fn to_str(self) -> &'static str {
		match self {
			Locale::cs_CZ => "cs_CZ",
			Locale::de_DE => "de_DE",
			Locale::el_GR => "el_GR",
			Locale::en_AU => "en_AU",
			Locale::en_GB => "en_GB",
			Locale::en_PH => "en_PH",
			Locale::en_PL => "en_PL",
			Locale::en_SG => "en_SG",
			Locale::en_US => "en_US",
			Locale::es_AR => "es_AR",
			Locale::es_ES => "es_ES",
			Locale::es_MX => "es_MX",
			Locale::fr_FR => "fr_FR",
			Locale::hu_HU => "hu_HU",
			Locale::id_ID => "id_ID",
			Locale::it_IT => "it_IT",
			Locale::ja_JP => "ja_JP",
			Locale::ko_KR => "ko_KR",
			Locale::ms_MY => "ms_MY",
			Locale::pl_PL => "pl_PL",
			Locale::pt_BR => "pt_BR",
			Locale::ro_RO => "ro_RO",
			Locale::ru_RU => "ru_RU",
			Locale::th_TH => "th_TH",
			Locale::tr_TR => "tr_TR",
			Locale::vn_VN => "vn_VN",
			Locale::zh_CN => "zh_CN",
			Locale::zh_MY => "zh_MY",
			Locale::zh_TW => "zh_TW",
		}
	}
}