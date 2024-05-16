use std::env;

mod models;
mod apis;
mod data_manager;

use apis::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WELCOME TO LEPRECHAUN");
    dotenvy::dotenv().expect(".env file not found");
    let vantage_alpha_key = env::var("VANTAGE_ALPHA_KEY").expect("VANTAGE_ALPHA_KEY not found in .env file");
    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");

    let api = Api::new_vantage_alpha(vantage_alpha_key);

    let data_manager = data_manager::DataManager::new(api, &postgres_url).await;
    let _ = data_manager.get_eod_data("IBM", None, None).await?;
    Ok(())
}
