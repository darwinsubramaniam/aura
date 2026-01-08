use std::collections::HashMap;

use crate::db::Db;
use crate::fiat::{frankfurter_api::FrankfurterApi, FiatService};
use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct FiatRate {
    pub id: i64,
    pub base_fiat_id: i64,
    #[sqlx(skip)]
    pub symbol: String,
    #[sqlx(skip)]
    pub name: String,
    pub date: NaiveDate,
    #[sqlx(json)]
    pub rates: HashMap<String, f64>,
}

// Get the rate for a specific fiat and date
pub async fn get_rate(db: &Db, base_fiat_id: i64, date: &NaiveDate) -> Result<FiatRate> {
    let db_result = sqlx::query_as::<sqlx::Sqlite, FiatRate>(
        "SELECT fiat_rate.*, fiat.symbol, fiat.name FROM fiat_rate 
        JOIN fiat ON fiat_rate.base_fiat_id = fiat.id 
        WHERE fiat_rate.base_fiat_id = ? AND fiat_rate.date = ?",
    )
    .bind(base_fiat_id)
    .bind(date)
    .fetch_one(&db.0)
    .await;

    match db_result {
        Ok(rate) => Ok(rate),
        Err(sqlx::Error::RowNotFound) => {
            let base_fiat = FiatService::get_fiat_by_id(db, base_fiat_id)
                .await
                .context("failed to get fiat by id")?;
            let base_symbol = base_fiat.symbol;
            let base_name = base_fiat.name;
            // forward the request to the Frankfurter API
            let rate = FrankfurterApi::get_latest_rates(&base_symbol.as_str(), Some(date))
                .await
                .context("External API :: Get Latest Rates :: Failed")?;
            // insert the rate into the database
            let mut fiat_rate = sqlx::query_as::<sqlx::Sqlite, FiatRate>(
                "INSERT INTO fiat_rate (base_fiat_id, date, rates) VALUES (?, ?, ?) RETURNING *",
            )
            .bind(base_fiat_id)
            .bind(date)
            .bind(sqlx::types::Json(&rate.rates))
            .fetch_one(&db.0)
            .await
            .context("Failed to insert rate into database")?;

            fiat_rate.symbol = base_symbol;
            fiat_rate.name = base_name;

            Ok(fiat_rate)
        }
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;
    use crate::fiat::FiatService;
    use chrono::NaiveDate;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

    async fn setup() -> Db {
        // create a test db
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(SqliteConnectOptions::new().in_memory(true))
            .await
            .context("failed to create in-memory database")
            .unwrap();

        let db = Db(pool);
        // create fiat_rate table
        sqlx::migrate!("./migrations")
            .run(&db.0)
            .await
            .context("failed to create fiat_rate table")
            .unwrap();

        // populate fiat table
        FiatService::update_currencies(&db).await.unwrap();

        db
    }

    #[tokio::test]
    async fn test_get_rate() {
        let db = setup().await;
        let rate = get_rate(&db, 10, &NaiveDate::from_ymd_opt(2026, 2, 1).unwrap()).await;
        assert!(!rate.is_err());
    }
}
