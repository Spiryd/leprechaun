use std::env;

mod models;
mod apis;
mod data_manager;

use apis::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WELCOME TO LEPRECHAUN");
    dotenvy::dotenv().expect(".env file not found");
    let vantage_alpha_key = env::var("VANTAGE_ALPHA_KEY").expect("VANTAGE_ALPHA_KEY not found in .env file");
    println!("VANTAGE_ALPHA_KEY: {:?}", vantage_alpha_key);

    let api = apis::vantage_alpha::VantageAlpha::new(vantage_alpha_key);
    let ohlcv = api.get_daily("IBM").await?;
    println!("{:?}", ohlcv);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    Ok(())
}
