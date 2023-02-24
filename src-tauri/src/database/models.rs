
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Jot {
    pub id: i64,
    pub text: String,
    pub img_path: Option<String>,
    #[serde(with = "json_time")]
    pub time_create: chrono::NaiveDateTime,
    #[serde(with = "json_time")]
    pub time_modified: chrono::NaiveDateTime
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub id: i64,
    pub title: String,
    pub color: Option<String>,
    pub priority: i64,
    #[serde(with = "json_time")]
    pub time_create: chrono::NaiveDateTime,
    #[serde(with = "json_time")]
    pub time_modified: chrono::NaiveDateTime
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct JotTag {
    pub id: i64,
    pub jot_id: i64,
    pub tag_id: i64,
}

pub fn time_to_json(t: chrono::NaiveDateTime) -> String {
	chrono::DateTime::<chrono::Utc>::from_utc(t, chrono::Utc).to_rfc3339()
}

mod json_time {
	use super::*;
	use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};

	pub fn serialize<S: Serializer>(time: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
		time_to_json(time.clone()).serialize(serializer)
	}
 
	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error> {
		let time: &str = Deserialize::deserialize(deserializer)?;
		Ok(chrono::DateTime::parse_from_rfc3339(time).map_err(D::Error::custom)?.naive_utc())
	}
}