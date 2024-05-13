use serde::{Deserialize, Deserializer};
use chrono::NaiveDate;

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct EodOhlcv {
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "date_string_to_navie_date")]
    time_stamp: NaiveDate,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
}

pub fn date_string_to_navie_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(NaiveDate::parse_from_str(&s, DATE_FORMAT).unwrap())
}

#[derive(sqlx::FromRow)]
pub struct EOD {
    ticker: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
    time_stamp: NaiveDate,
}
