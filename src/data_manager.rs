use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::apis::Api;


pub struct DataManager {
    api: Api,
    pool: Pool<Postgres>
}

impl DataManager {
    pub async fn new(api: Api, database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await.unwrap();
        Self { api, pool }
    }
}
