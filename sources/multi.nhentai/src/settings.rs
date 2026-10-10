use aidoku::{
	alloc::{string::String, vec::Vec},
	imports::defaults::defaults_get,
};
const TITLE_PREFERENCE_KEY: &str = "titlePreference";
const LANGUAGE_KEY: &str = "language";
const BLOCKLIST_KEY: &str = "blocklist";
const WANTED_KEY: &str = "wantedTags";
const SORT_KEY: &str = "defaultSort";
const LIST_VIEWER_KEY: &str = "isListView";

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TitlePreference {
	#[default]
	English,
	Japanese,
}

impl From<String> for TitlePreference {
	fn from(value: String) -> Self {
		match value.as_str() {
			"japanese" => Self::Japanese,
			"english" => Self::English,
			_ => Self::English,
		}
	}
}

pub fn get_title_preference() -> TitlePreference {
	defaults_get::<String>(TITLE_PREFERENCE_KEY)
		.map(TitlePreference::from)
		.unwrap_or_default()
}

pub fn get_language() -> Option<String> {
	// falls back to English when no language is set, so results are never "all languages"
	let lang: String = defaults_get::<String>(LANGUAGE_KEY)
		.and_then(|lang| match lang.as_str() {
			"en" => Some("english".into()),
			"ja" => Some("japanese".into()),
			"zh" => Some("chinese".into()),
			_ => None,
		})
		.unwrap_or_else(|| "english".into());
	Some(lang)
}

pub fn get_blocklist() -> Vec<String> {
	defaults_get::<Vec<String>>(BLOCKLIST_KEY)
		.unwrap_or_default()
		.into_iter()
		.map(|s| s.trim().to_lowercase())
		.filter(|s| !s.is_empty())
		.collect()
}

pub fn get_list_viewer() -> bool {
	defaults_get(LIST_VIEWER_KEY).unwrap_or(false)
}

pub fn get_wanted_tags() -> Vec<String> {
	defaults_get::<Vec<String>>(WANTED_KEY)
		.unwrap_or_default()
		.into_iter()
		.map(|s| s.trim().to_lowercase())
		.filter(|s| !s.is_empty())
		.collect()
}

pub fn get_default_sort() -> &'static str {
	match defaults_get::<String>(SORT_KEY).as_deref() {
		Some("popular-today") => "popular-today",
		Some("popular-week") => "popular-week",
		Some("popular") => "popular",
		_ => "date",
	}
}
