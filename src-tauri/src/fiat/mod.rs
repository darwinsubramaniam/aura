pub mod command;
mod frankfurter_api;

use serde::Serialize;

use crate::{sys_tracker::SysTracker, utils, Db};
pub struct FiatService {}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Fiat {
    pub id: i64,
    pub symbol: String,
    pub name: String,
}

const FIAT_SYS_TRACKER_NAME: &str = "fiat";

impl FiatService {
    // update database with supported currencies symbols and names
    pub async fn update_currencies(db: &Db) -> Result<(), String> {
        // last updated at
        let last_updated_at = SysTracker::get_last_updated_at(FIAT_SYS_TRACKER_NAME, &db)
            .await
            .map_err(|e| format!("failed to get last updated at: {e}"))?;

        let require_update = utils::require_update(last_updated_at, chrono::Duration::hours(24));

        if !require_update {
            println!("fiat table is up to date");
            return Ok(());
        }

        // 1. Get available currencies from Frankfurter API
        let currencies = frankfurter_api::FrankfurterApi::get_available_currencies()
            .await
            .map_err(|e| format!("failed to get available currencies: {e}"))?;

        let tx =
            db.0.begin()
                .await
                .map_err(|e| format!("failed to begin transaction: {e}"))?;

        // 1. Delete all rows from fiat table - this is to ensure that we are not adding duplicate rows
        sqlx::query("DELETE FROM fiat")
            .execute(&db.0)
            .await
            .map_err(|e| format!("failed to delete from fiat table: {e}"))?;

        // 2. Update database with supported currencies symbols and names
        for currency in currencies {
            let symbol = currency.symbol;
            let name = currency.name;
            sqlx::query("INSERT INTO fiat (symbol, name) VALUES (?, ?)")
                .bind(symbol)
                .bind(name)
                .execute(&db.0)
                .await
                .map_err(|e| format!("failed to insert into fiat table: {e}"))?;
        }

        // 3. Update SysTracker
        SysTracker::update_last_updated_at(FIAT_SYS_TRACKER_NAME, &db)
            .await
            .map_err(|e| format!("SysTracker update error: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| format!("failed to commit transaction: {e}"))?;
        Ok(())
    }

    pub async fn get_fiat(
        db: &Db,
        symbol: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Fiat>, String> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let base_query = if let Some(symbol) = symbol {
            sqlx::query_as::<_, Fiat>("SELECT * FROM fiat WHERE symbol = ? LIMIT ? OFFSET ?")
                .bind(symbol)
        } else {
            sqlx::query_as::<_, Fiat>("SELECT * FROM fiat LIMIT ? OFFSET ?")
        };
        let fiat = base_query
            .bind(limit)
            .bind(offset)
            .fetch_all(&db.0)
            .await
            .map_err(|e| format!("failed to get fiat by symbol: {e}"))?;

        Ok(fiat)
    }
}
