use crate::models::EodOhlcv;
use crate::ApiGetters;

#[allow(unused_variables)]

pub struct XtbApi {
    api_key: String,
}

impl XtbApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl ApiGetters for XtbApi {
    async fn get_daily(&self, symbol: &str) -> Result<Vec<EodOhlcv>, Box<dyn std::error::Error>> {
        todo!()
    }
}
