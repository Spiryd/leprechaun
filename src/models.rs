use serde::{Deserialize, Deserializer};
use chrono::{DateTime, NaiveDate, Utc};

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Deserialize)]
pub struct OHLCV {
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "date_string_to_datetime")]
    time_stamp: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
}

pub fn date_string_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let nd = NaiveDate::parse_from_str(&s, DATE_FORMAT).unwrap();
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(nd.and_hms_opt(0, 0, 0).unwrap(), Utc))

}
