pub mod command;
use crate::db::{Db, RowId, StringRowId};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqliteQueryResult};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum RampKind {
    Deposit,
    Withdraw,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FiatRamp {
    pub id: StringRowId,
    pub fiat_id: i64,
    pub fiat_amount: f64,
    pub ramp_date: chrono::NaiveDate,
    pub kind: RampKind,
    pub via_exchange: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    #[serde(default)]
    pub fiat_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFiatRamp {
    pub fiat_id: RowId,
    pub fiat_amount: f64,
    pub ramp_date: chrono::NaiveDate,
    pub via_exchange: String,
    pub kind: RampKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FiatRampPagination {
    pub total_count: i64,
    pub fiat_ramps: Vec<FiatRamp>,
}

pub struct FiatRampService {}

impl FiatRampService {
    /// Create a new fiat ramp
    pub async fn create_fiat_ramp(
        create_fiat_ramp: CreateFiatRamp,
        db: &Db,
    ) -> Result<String, String> {
        let id = Uuid::now_v7().to_string();
        let mut tx =
            db.0.begin()
                .await
                .map_err(|e| format!("failed to begin transaction: {e}"))?;

        sqlx::query("INSERT INTO fiat_ramp (id, fiat_id, fiat_amount, ramp_date, via_exchange, kind) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(create_fiat_ramp.fiat_id)
        .bind(create_fiat_ramp.fiat_amount)
        .bind(create_fiat_ramp.ramp_date)
        .bind(create_fiat_ramp.via_exchange)
        .bind(create_fiat_ramp.kind)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("failed to insert into fiat_ramp table: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| format!("failed to commit transaction: {e}"))?;
        Ok(id)
    }

    /// Get all fiat ramps
    pub async fn get_fiat_ramp(
        limit: u32,
        offset: u32,
        query: Option<String>,
        db: &Db,
    ) -> Result<FiatRampPagination, String> {
        let query_filter = query.unwrap_or_default();
        let total_count = sqlx::query_scalar("SELECT COUNT(*) FROM fiat_ramp LEFT JOIN fiat ON fiat_ramp.fiat_id = fiat.id WHERE via_exchange LIKE ? OR kind LIKE ? OR CAST(fiat_amount AS TEXT) LIKE ? OR fiat.symbol LIKE ?")
            .bind(format!("%{}%", query_filter))
            .bind(format!("%{}%", query_filter))
            .bind(format!("%{}%", query_filter))
            .bind(format!("%{}%", query_filter))
            .fetch_one(&db.0)
            .await
            .map_err(|e| format!("failed to get total count: {e}"))?;
        let result =
            sqlx::query_as::<sqlx::Sqlite, FiatRamp>("SELECT fiat_ramp.*, fiat.symbol as fiat_symbol FROM fiat_ramp LEFT JOIN fiat ON fiat_ramp.fiat_id = fiat.id WHERE via_exchange LIKE ? OR kind LIKE ? OR CAST(fiat_amount AS TEXT) LIKE ? OR fiat.symbol LIKE ? LIMIT ? OFFSET ?")
                .bind(format!("%{}%", query_filter))
                .bind(format!("%{}%", query_filter))
                .bind(format!("%{}%", query_filter))
                .bind(format!("%{}%", query_filter))
                .bind(limit)
                .bind(offset)
                .fetch_all(&db.0)
                .await
                .map_err(|e| format!("failed to select from fiat_ramp table: {e}"))?;
        Ok(FiatRampPagination {
            total_count,
            fiat_ramps: result,
        })
    }

    /// Update a fiat ramp
    pub async fn update_fiat_ramp(fiat_ramp: FiatRamp, db: &Db) -> Result<u64, String> {
        let mut tx =
            db.0.begin()
                .await
                .map_err(|e| format!("failed to begin transaction: {e}"))?;

        let result: SqliteQueryResult = sqlx::query("UPDATE fiat_ramp SET fiat_id = ?, fiat_amount = ?, date = ?, via_exchange = ?, kind = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(fiat_ramp.fiat_id)
            .bind(fiat_ramp.fiat_amount)
            .bind(fiat_ramp.ramp_date)
            .bind(fiat_ramp.via_exchange)
            .bind(fiat_ramp.kind)
            .bind(fiat_ramp.id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("failed to update fiat_ramp table: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| format!("failed to commit transaction: {e}"))?;
        Ok(result.rows_affected())
    }

    /// Delete a fiat ramp
    pub async fn delete_fiat_ramp(id: StringRowId, db: &Db) -> Result<u64, String> {
        let mut tx =
            db.0.begin()
                .await
                .map_err(|e| format!("failed to begin transaction: {e}"))?;

        let result: SqliteQueryResult = sqlx::query("DELETE FROM fiat_ramp WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("failed to delete from fiat_ramp table: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| format!("failed to commit transaction: {e}"))?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;
    use sqlx::sqlite::SqlitePool;

    async fn init_db() -> Db {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let db = Db(pool);

        // create fiat table
        sqlx::query(include_str!("../.././migrations/3_create_fiat.sql"))
            .execute(&db.0)
            .await
            .map_err(|e| format!("failed to create fiat table: {e}"))
            .unwrap();

        // create fiat_ramp table
        sqlx::query(include_str!(
            "../.././migrations/5_fiat_ramp_transaction.sql"
        ))
        .execute(&db.0)
        .await
        .map_err(|e| format!("failed to create fiat_ramp table: {e}"))
        .unwrap();
        db
    }

    #[tokio::test]
    async fn test_create_fiat_ramp() {
        let db = init_db().await;
        let create_fiat_ramp = CreateFiatRamp {
            fiat_id: 1,
            fiat_amount: 100.0,
            ramp_date: chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            via_exchange: "coinbase".to_string(),
            kind: RampKind::Deposit,
        };
        let result = FiatRampService::create_fiat_ramp(create_fiat_ramp, &db).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_fiat_ramp() {
        let db = init_db().await;
        //create a fiat ramp 11 mock data
        for i in 1..=11 {
            let create_fiat_ramp = CreateFiatRamp {
                fiat_id: i,
                fiat_amount: 100.0,
                ramp_date: chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                via_exchange: "coinbase".to_string(),
                kind: RampKind::Deposit,
            };
            let _ = FiatRampService::create_fiat_ramp(create_fiat_ramp, &db).await;
        }

        let result = FiatRampService::get_fiat_ramp(10, 0, None, &db)
            .await
            .unwrap();
        assert!(
            result.fiat_ramps.len() == 10,
            "After offset 0, expected 10 fiat ramps, got {}",
            result.fiat_ramps.len()
        );
        assert!(
            result.total_count == 11,
            "After offset 0, expected 11 total count, got {}",
            result.total_count
        );

        let result = FiatRampService::get_fiat_ramp(10, 10, None, &db)
            .await
            .unwrap();
        assert!(
            result.fiat_ramps.len() == 1,
            "After offset 10, expected 1 fiat ramp, got {}",
            result.fiat_ramps.len()
        );
        assert!(
            result.total_count == 11,
            "After offset 10, expected 11 total count, got {}",
            result.total_count
        );
    }

    #[tokio::test]
    async fn test_update_fiat_ramp() {
        let db = init_db().await;
        let create_fiat_ramp = CreateFiatRamp {
            fiat_id: 1,
            fiat_amount: 100.0,
            ramp_date: chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            via_exchange: "coinbase".to_string(),
            kind: RampKind::Deposit,
        };
        let id = FiatRampService::create_fiat_ramp(create_fiat_ramp, &db)
            .await
            .unwrap();
        // assert id is kind of uuid v7
        assert!(Uuid::from_str(&id).is_ok());

        let fiat_ramp = FiatRamp {
            id,
            fiat_id: 1,
            fiat_amount: 200.0,
            ramp_date: chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            via_exchange: "coinbase".to_string(),
            kind: RampKind::Deposit,
            created_at: chrono::NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap(),
            updated_at: chrono::NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap(),
            fiat_symbol: None,
        };
        let result = FiatRampService::update_fiat_ramp(fiat_ramp, &db).await;
        assert!(result.unwrap() == 1);
    }

    #[tokio::test]
    async fn test_delete_fiat_ramp() {
        let db = init_db().await;
        let create_fiat_ramp = CreateFiatRamp {
            fiat_id: 1,
            fiat_amount: 100.0,
            ramp_date: chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            via_exchange: "coinbase".to_string(),
            kind: RampKind::Deposit,
        };
        let id = FiatRampService::create_fiat_ramp(create_fiat_ramp, &db)
            .await
            .unwrap();
        // assert id is kind of uuid v7
        assert!(Uuid::from_str(&id).is_ok());

        let result = FiatRampService::delete_fiat_ramp(id, &db).await;
        assert!(result.unwrap() == 1);
    }
}
