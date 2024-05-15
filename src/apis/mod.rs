use crate::models::EodOhlcv;

pub mod vantage_alpha;

pub enum Api {
    VantageAlpha(vantage_alpha::VantageAlpha),
}

impl Api {
    pub fn new_vantage_alpha(api_key: String) -> Self {
        Api::VantageAlpha(vantage_alpha::VantageAlpha::new(api_key))
    }

    pub async fn get_eod(&self, symbol: &str) -> Result<Vec<EodOhlcv>, Box<dyn std::error::Error>> {
        match self {
            Api::VantageAlpha(api) => api.get_eod(symbol).await,
        }
    }
}

pub trait ApiGetters {
    async fn get_eod(&self, symbol: &str) -> Result<Vec<EodOhlcv>, Box<dyn std::error::Error>>;
}