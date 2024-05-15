use crate::apis::ApiGetters;
use crate::models::EodOhlcv;

pub struct VantageAlpha {
    api_key: String,
}

impl VantageAlpha {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl ApiGetters for VantageAlpha {
    async fn get_eod(&self, symbol: &str) -> Result<Vec<EodOhlcv>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let rsp = client.get("https://www.alphavantage.co/query?").query(&[("function", "TIME_SERIES_DAILY"), ("symbol", symbol), ("outputsize", "full"), ("apikey", &self.api_key),("datatype", "csv")]).send().await?;
        let body = rsp.text().await?;
        let mut rdr = csv::Reader::from_reader(body.as_bytes());
        let mut ohlcv = Vec::new();
        for result in rdr.deserialize() {
            let record: EodOhlcv = result?;
            ohlcv.push(record);
        }
        Ok(ohlcv)
    }
}
