use std::env;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WELCOME TO LEPRECHAUN");
    dotenvy::dotenv().expect(".env file not found");
    let vantage_alpha_key = env::var("VANTAGE_ALPHA_KEY").expect("VANTAGE_ALPHA_KEY not found in .env file");
    println!("VANTAGE_ALPHA_KEY: {:?}", vantage_alpha_key);
    let client = reqwest::Client::new();
    let rsp = client.get("https://www.alphavantage.co/query?").query(&[("function", "TIME_SERIES_DAILY"), ("symbol", "IBM"), ("outputsize", "full"), ("apikey", &vantage_alpha_key),("datatype", "csv")]).send().await?;
    let body = rsp.text().await?;
    let mut rdr = csv::Reader::from_reader(body.as_bytes());
    for result in rdr.deserialize() {
        let record: models::OHLCV = result?;
        println!("{:?}", record);
    }
    Ok(())
}
