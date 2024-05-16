use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct EodOhlcv {
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "date_string_to_navie_date")]
    pub date_stamp: NaiveDate,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

pub fn date_string_to_navie_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(NaiveDate::parse_from_str(&s, DATE_FORMAT).unwrap())
}

#[derive(sqlx::FromRow, Debug)]
pub struct EodCacheRegistery {
    pub ticker: String,
    pub end_date: NaiveDate,
    pub start_date: NaiveDate,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Eod {
    pub ticker: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub date_stamp: NaiveDate,
}
