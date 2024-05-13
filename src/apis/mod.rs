use crate::models::EodOhlcv;

pub mod vantage_alpha;
pub mod xtb_api;

pub enum Api {
    VantageAlpha(vantage_alpha::VantageAlpha),
    XtbApi(xtb_api::XtbApi),
}

impl Api {
    pub fn new_vantage_alpha(api_key: String) -> Self {
        Api::VantageAlpha(vantage_alpha::VantageAlpha::new(api_key))
    }

    pub async fn get_daily(&self, symbol: &str) -> Result<Vec<EodOhlcv>, Box<dyn std::error::Error>> {
        match self {
            Api::VantageAlpha(api) => api.get_daily(symbol).await,
            Api::XtbApi(api) => api.get_daily(symbol).await,
        }
    }
}

pub trait ApiGetters {
    async fn get_daily(&self, symbol: &str) -> Result<Vec<EodOhlcv>, Box<dyn std::error::Error>>;
}