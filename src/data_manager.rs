use chrono::NaiveDate;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::apis::Api;
use crate::models::{Eod, EodCacheRegistery};

pub struct DataManager {
    api: Api,
    pool: Pool<Postgres>,
}

impl DataManager {
    pub async fn new(api: Api, database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .unwrap();
        Self { api, pool }
    }

    pub async fn get_eod_data(
        &self,
        ticker: &str,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<Vec<Eod>, Box<dyn std::error::Error>> {
        let exists: (i64,) =
            sqlx::query_as("SELECT COUNT(1) FROM eod_cache_registery WHERE ticker = $1")
                .bind(ticker)
                .fetch_one(&self.pool)
                .await?;
        if exists.0 == 0 {
            let ohlcv = self.api.get_eod(ticker).await?;
            for record in &ohlcv {
                sqlx::query("INSERT INTO eod (ticker, date_stamp, open, high, low, close, volume) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                    .bind(ticker)
                    .bind(record.date_stamp)
                    .bind(record.open)
                    .bind(record.high)
                    .bind(record.low)
                    .bind(record.close)
                    .bind(record.volume as i64)
                    .execute(&self.pool).await?;
            }
            sqlx::query("INSERT INTO eod_cache_registery (ticker, start_date, end_date) VALUES ($1, $2, $3)")
                .bind(ticker)
                .bind(ohlcv.first().unwrap().date_stamp)
                .bind(ohlcv.last().unwrap().date_stamp)
                .execute(&self.pool).await?;
        }

        let cahce_registery_entery = sqlx::query_as::<_, EodCacheRegistery>(
            "SELECT * FROM eod_cache_registery WHERE ticker = $1",
        )
        .bind(ticker)
        .fetch_one(&self.pool)
        .await?;
        match start_date {
            Some(start_date) => match end_date {
                Some(end_date) => {
                    if start_date < cahce_registery_entery.start_date
                        || end_date > cahce_registery_entery.end_date
                    {
                        let ohlcv = self.api.get_eod("IBM").await?;
                        for record in &ohlcv {
                            if cahce_registery_entery.end_date < record.date_stamp
                                || cahce_registery_entery.start_date > record.date_stamp
                            {
                                sqlx::query("INSERT INTO eod (ticker, date_stamp, open, high, low, close, volume) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                                                .bind(ticker)
                                                .bind(record.date_stamp)
                                                .bind(record.open)
                                                .bind(record.high)
                                                .bind(record.low)
                                                .bind(record.close)
                                                .bind(record.volume as i64)
                                                .execute(&self.pool).await?;
                            }
                        }
                        sqlx::query("UPDATE eod_cache_registery SET start_date = $1, end_date = $2 WHERE ticker = $3")
                                        .bind(ohlcv.first().unwrap().date_stamp)
                                        .bind(ohlcv.last().unwrap().date_stamp)
                                        .bind(ticker)
                                        .execute(&self.pool).await?;
                        let mut eod = Vec::new();
                        for record in &ohlcv {
                            if record.date_stamp >= start_date && record.date_stamp <= end_date {
                                eod.push(Eod {
                                    ticker: ticker.to_string(),
                                    open: record.open,
                                    high: record.high,
                                    low: record.low,
                                    close: record.close,
                                    volume: record.volume as i64,
                                    date_stamp: record.date_stamp,
                                });
                            }
                        }
                        return Ok(eod);
                    } else {
                        let eod = sqlx::query_as::<_, Eod>("SELECT * FROM eod WHERE ticker = $1 AND date_stamp >= $2 AND date_stamp <= $3")
                                        .bind(ticker)
                                        .bind(start_date)
                                        .bind(end_date)
                                        .fetch_all(&self.pool).await?;
                        return Ok(eod);
                    }
                }
                None => {
                    if start_date < cahce_registery_entery.start_date {
                        let ohlcv = self.api.get_eod("IBM").await?;
                        for record in &ohlcv {
                            if cahce_registery_entery.start_date > record.date_stamp {
                                sqlx::query("INSERT INTO eod (ticker, date_stamp, open, high, low, close, volume) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                                                .bind(ticker)
                                                .bind(record.date_stamp)
                                                .bind(record.open)
                                                .bind(record.high)
                                                .bind(record.low)
                                                .bind(record.close)
                                                .bind(record.volume as i64)
                                                .execute(&self.pool).await?;
                            }
                        }
                        sqlx::query(
                            "UPDATE eod_cache_registery SET start_date = $1 WHERE ticker = $2",
                        )
                        .bind(ohlcv.first().unwrap().date_stamp)
                        .bind(ticker)
                        .execute(&self.pool)
                        .await?;
                        let mut eod = Vec::new();
                        for record in &ohlcv {
                            if record.date_stamp >= start_date {
                                eod.push(Eod {
                                    ticker: ticker.to_string(),
                                    open: record.open,
                                    high: record.high,
                                    low: record.low,
                                    close: record.close,
                                    volume: record.volume as i64,
                                    date_stamp: record.date_stamp,
                                });
                            }
                        }
                        return Ok(eod);
                    } else {
                        let eod = sqlx::query_as::<_, Eod>(
                            "SELECT * FROM eod WHERE ticker = $1 AND date_stamp >= $2",
                        )
                        .bind(ticker)
                        .bind(start_date)
                        .fetch_all(&self.pool)
                        .await?;
                        return Ok(eod);
                    }
                }
            },
            None => match end_date {
                Some(end_date) => {
                    if end_date > cahce_registery_entery.end_date {
                        let ohlcv = self.api.get_eod("IBM").await?;
                        for record in &ohlcv {
                            if cahce_registery_entery.end_date < record.date_stamp {
                                sqlx::query("INSERT INTO eod (ticker, date_stamp, open, high, low, close, volume) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                                                .bind(ticker)
                                                .bind(record.date_stamp)
                                                .bind(record.open)
                                                .bind(record.high)
                                                .bind(record.low)
                                                .bind(record.close)
                                                .bind(record.volume as i64)
                                                .execute(&self.pool).await?;
                            }
                        }
                        sqlx::query(
                            "UPDATE eod_cache_registery SET end_date = $1 WHERE ticker = $2",
                        )
                        .bind(ohlcv.last().unwrap().date_stamp)
                        .bind(ticker)
                        .execute(&self.pool)
                        .await?;
                        let mut eod = Vec::new();
                        for record in &ohlcv {
                            if record.date_stamp <= end_date {
                                eod.push(Eod {
                                    ticker: ticker.to_string(),
                                    open: record.open,
                                    high: record.high,
                                    low: record.low,
                                    close: record.close,
                                    volume: record.volume as i64,
                                    date_stamp: record.date_stamp,
                                });
                            }
                        }
                        return Ok(eod);
                    } else {
                        let eod = sqlx::query_as::<_, Eod>(
                            "SELECT * FROM eod WHERE ticker = $1 AND date_stamp <= $2",
                        )
                        .bind(ticker)
                        .bind(end_date)
                        .fetch_all(&self.pool)
                        .await?;
                        return Ok(eod);
                    }
                }
                None => {
                    let eod = sqlx::query_as::<_, Eod>("SELECT * FROM eod WHERE ticker = $1")
                        .bind(ticker)
                        .fetch_all(&self.pool)
                        .await?;
                    return Ok(eod);
                }
            },
        }
    }
}
