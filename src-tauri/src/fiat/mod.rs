pub mod command;
pub mod frankfurter_api;
use crate::{sys_tracker::SysTracker, utils, Db};
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
pub struct FiatService {}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Fiat {
    pub id: i64,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct FiatRate {
    pub id: i64,
    pub base_fiat_id: i64,
    pub date: chrono::NaiveDate,
    pub rates: HashMap<String, f64>,
}

const FIAT_SYS_TRACKER_NAME: &str = "fiat";

impl FiatService {
    // update database with supported currencies symbols and names
    pub async fn update_currencies(db: &Db) -> Result<()> {
        // last updated at
        let last_updated_at = SysTracker::get_last_updated_at(FIAT_SYS_TRACKER_NAME, &db)
            .await
            .context("failed to get last updated at")?;

        let require_update = utils::require_update(last_updated_at, chrono::Duration::hours(24));

        if !require_update {
            return Ok(());
        }

        let current_update_at = chrono::Local::now().naive_local();

        // 1. Get available currencies from Frankfurter API
        let currencies = frankfurter_api::FrankfurterApi::get_available_currencies()
            .await
            .context("failed to get available currencies")?;

        let mut tx = db.0.begin().await.context("failed to begin transaction")?;

        // 2. Update database with supported currencies symbols and names
        for currency in currencies {
            let symbol = currency.symbol;
            let name = currency.name;
            sqlx::query("INSERT OR REPLACE INTO fiat (symbol, name, updated_at) VALUES (?, ?, ?)")
                .bind(symbol)
                .bind(name)
                .bind(current_update_at)
                .execute(&mut *tx)
                .await
                .context("failed to insert into fiat table")?;
        }

        tx.commit().await.context("failed to commit transaction")?;

        // 3. Update SysTracker
        SysTracker::update_last_updated_at(FIAT_SYS_TRACKER_NAME, &db)
            .await
            .context("SysTracker update error")?;
        Ok(())
    }

    pub async fn get_fiat(
        db: &Db,
        symbol: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Fiat>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let base_query = if let Some(symbol) = symbol {
            sqlx::query_as::<_, Fiat>("SELECT * FROM fiat WHERE symbol LIKE ? LIMIT ? OFFSET ?")
                .bind(format!("%{}%", symbol))
        } else {
            sqlx::query_as::<_, Fiat>("SELECT * FROM fiat LIMIT ? OFFSET ?")
        };
        let fiat = base_query
            .bind(limit)
            .bind(offset)
            .fetch_all(&db.0)
            .await
            .context("failed to get fiat by symbol")?;

        Ok(fiat)
    }

    pub async fn get_all_fiat(db: &Db) -> Result<Vec<Fiat>> {
        let fiat = sqlx::query_as::<_, Fiat>("SELECT * FROM fiat")
            .fetch_all(&db.0)
            .await
            .context("failed to get all fiat")?;
        Ok(fiat)
    }

    pub async fn get_total_count(db: &Db) -> Result<i64> {
        let total_count = sqlx::query_scalar("SELECT COUNT(*) FROM fiat")
            .fetch_one(&db.0)
            .await
            .context("failed to get total count")?;
        Ok(total_count)
    }

    pub async fn get_fiat_by_id(db: &Db, id: i64) -> Result<Fiat> {
        let fiat = sqlx::query_as::<_, Fiat>("SELECT * FROM fiat WHERE id = ?")
            .bind(id)
            .fetch_one(&db.0)
            .await
            .context("failed to get fiat by id")?;
        Ok(fiat)
    }
}
