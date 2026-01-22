pub mod command;
use crate::db::{Db, RowId, StringRowId};
use crate::utils::pagination_model::{SortDirection, SortOptions};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqliteQueryResult};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum RampKind {
    Deposit,
    Withdraw,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FiatRamp {
    pub id: StringRowId,
    pub fiat_id: RowId,
    pub fiat_amount: f64,
    pub ramp_date: chrono::NaiveDate,
    pub kind: RampKind,
    pub via_exchange: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFiatRamp {
    pub id: StringRowId,
    pub fiat_id: Option<RowId>,
    pub fiat_amount: Option<f64>,
    pub ramp_date: Option<chrono::NaiveDate>,
    pub kind: Option<RampKind>,
    pub via_exchange: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fiat_ramps: Vec<FiatRampWithConversionView>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FiatRampSummary {
    pub total_deposit: f64,
    pub total_withdraw: f64,
    pub fiat_symbol: String,
    pub fiat_name: String,
    pub data: HashMap<String, f64>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FiatRampWithConversionView {
    pub fiat_ramp_id: StringRowId,
    pub from_fiat_id: RowId,
    pub from_fiat_symbol: String,
    pub from_fiat_name: String,
    pub to_fiat_id: RowId,
    pub to_fiat_symbol: String,
    pub to_fiat_name: String,
    pub conversion_rate: Option<f64>,
    pub ramp_date: NaiveDate,
    pub fiat_amount: f64,
    pub kind: RampKind,
    pub via_exchange: String,
    #[sqlx(default)]
    pub is_estimated: bool,
    #[sqlx(default)]
    pub is_non_working_day: bool,
    #[sqlx(default)]
    pub non_working_day_reason: Option<String>,
    pub converted_amount: Option<f64>,
}

pub struct FiatRampService {}

impl FiatRampService {
    /// Create a new fiat ramp
    pub async fn create(create_fiat_ramp: CreateFiatRamp, db: &Db) -> Result<StringRowId, String> {
        let id = Uuid::now_v7().to_string();
        let mut tx =
            db.0.begin()
                .await
                .map_err(|e| format!("failed to begin transaction: {e}"))?;

        sqlx::query(
            r#"
            INSERT INTO fiat_ramp
            (id, fiat_id, fiat_amount, ramp_date, via_exchange, kind)
            VALUES (?, ?, ?, ?, ?, ?)
        "#,
        )
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

    /// Create multiple fiat ramps and return all IDs
    pub async fn create_bulk(ramps: Vec<CreateFiatRamp>, db: &Db) -> Result<Vec<FiatRamp>, String> {
        let mut tx =
            db.0.begin()
                .await
                .map_err(|e| format!("failed to begin transaction: {e}"))?;

        let mut fiat_ramps: Vec<FiatRamp> = Vec::with_capacity(ramps.len());
        for ramp in ramps {
            let id = Uuid::now_v7().to_string();
            sqlx::query(
                r#"
                INSERT INTO fiat_ramp
                (id, fiat_id, fiat_amount, ramp_date, via_exchange, kind)
                VALUES (?, ?, ?, ?, ?, ?)
            "#,
            )
            .bind(&id)
            .bind(ramp.fiat_id)
            .bind(ramp.fiat_amount)
            .bind(ramp.ramp_date)
            .bind(&ramp.via_exchange)
            .bind(&ramp.kind)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("failed to insert fiat ramp: {e}"))?;

            fiat_ramps.push(FiatRamp {
                id,
                fiat_id: ramp.fiat_id,
                fiat_amount: ramp.fiat_amount,
                ramp_date: ramp.ramp_date,
                via_exchange: ramp.via_exchange,
                kind: ramp.kind,
            });
        }

        tx.commit()
            .await
            .map_err(|e| format!("failed to commit transaction: {e}"))?;

        Ok(fiat_ramps)
    }

    /// Get all fiat ramps
    pub async fn get(
        limit: u32,
        offset: u32,
        query: Option<String>,
        sort: Option<SortOptions>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        db: &Db,
    ) -> Result<FiatRampPagination, String> {
        let query_filter = query.unwrap_or_default();
        let total_count = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM fiat_ramp_view
            WHERE
            (via_exchange LIKE ?
            OR kind LIKE ?
            OR CAST(fiat_amount AS TEXT) LIKE ?
            OR from_fiat_symbol LIKE ?
            OR from_fiat_name LIKE ?)
            AND (ramp_date >= ? OR ? IS NULL)
            AND (ramp_date <= ? OR ? IS NULL)
        "#,
        )
        .bind(format!("%{}%", query_filter))
        .bind(format!("%{}%", query_filter))
        .bind(format!("%{}%", query_filter))
        .bind(format!("%{}%", query_filter))
        .bind(format!("%{}%", query_filter))
        .bind(start_date)
        .bind(start_date)
        .bind(end_date)
        .bind(end_date)
        .fetch_one(&db.0)
        .await
        .map_err(|e| format!("failed to get total count: {e}"))?;

        // Build ORDER BY clause with whitelisted columns
        let order_by = match sort {
            Some(SortOptions {
                column: Some(col),
                direction,
            }) => {
                // Whitelist allowed columns to prevent SQL injection
                let valid_column = match col.as_str() {
                    "ramp_date" => "ramp_date",
                    "fiat_amount" => "fiat_amount",
                    "converted_amount" => "converted_amount",
                    "via_exchange" => "via_exchange",
                    "kind" => "kind",
                    _ => "ramp_date", // Default fallback
                };
                let dir = match direction.unwrap_or_default() {
                    SortDirection::Asc => "ASC",
                    SortDirection::Desc => "DESC",
                };
                format!("ORDER BY {} {}", valid_column, dir)
            }
            _ => "ORDER BY ramp_date DESC".to_string(), // Default sort
        };

        let sql = format!(
            r#"
                SELECT * FROM fiat_ramp_view
                WHERE (via_exchange LIKE ?
                OR kind LIKE ?
                OR CAST(fiat_amount AS TEXT) LIKE ?
                OR from_fiat_symbol LIKE ?
                OR from_fiat_name LIKE ?)
                AND (ramp_date >= ? OR ? IS NULL)
                AND (ramp_date <= ? OR ? IS NULL)
                {}
                LIMIT ?
                OFFSET ?
            "#,
            order_by
        );

        let result = sqlx::query_as::<sqlx::Sqlite, FiatRampWithConversionView>(&sql)
            .bind(format!("%{}%", query_filter))
            .bind(format!("%{}%", query_filter))
            .bind(format!("%{}%", query_filter))
            .bind(format!("%{}%", query_filter))
            .bind(format!("%{}%", query_filter))
            .bind(start_date)
            .bind(start_date)
            .bind(end_date)
            .bind(end_date)
            .bind(limit)
            .bind(offset)
            .fetch_all(&db.0)
            .await
            .map_err(|e| format!("failed to select from fiat_ramp_view: {e}"))?;
        Ok(FiatRampPagination {
            total_count,
            fiat_ramps: result,
        })
    }

    pub async fn get_summary(
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        db: &Db,
    ) -> Result<FiatRampSummary, String> {
        let (total_deposit, total_withdraw): (Option<f64>, Option<f64>) = sqlx::query_as(
            r#"
            SELECT
                SUM(CASE WHEN kind = 'deposit' THEN converted_amount ELSE 0.0 END) as total_deposit,
                SUM(CASE WHEN kind = 'withdraw' THEN converted_amount ELSE 0.0 END) as total_withdraw
            FROM fiat_ramp_view
            WHERE (ramp_date >= ? OR ? IS NULL)
            AND (ramp_date <= ? OR ? IS NULL)
            "#,
        )
        .bind(start_date)
        .bind(start_date)
        .bind(end_date)
        .bind(end_date)
        .fetch_one(&db.0)
        .await
        .map_err(|e| format!("failed to get summary: {e}"))?;

        let target_fiat_info = sqlx::query(
            r#"
            SELECT symbol, name
            FROM fiat
            JOIN user_settings ON user_settings.default_fiat_id = fiat.id
            WHERE user_settings.id = 1
            "#,
        )
        .fetch_optional(&db.0)
        .await
        .map_err(|e| format!("failed to get target fiat info: {e}"))?;

        let (fiat_symbol, fiat_name) = match target_fiat_info {
            Some(row) => {
                use sqlx::Row;
                (row.get::<String, _>("symbol"), row.get::<String, _>("name"))
            }
            None => ("?".to_string(), "Unknown".to_string()),
        };

        Ok(FiatRampSummary {
            total_deposit: total_deposit.unwrap_or(0.0),
            total_withdraw: total_withdraw.unwrap_or(0.0),
            fiat_symbol,
            fiat_name,
            data: HashMap::new(),
        })
    }

    /// Get the min and max date of all fiat ramps
    pub async fn get_date_range(db: &Db) -> Result<(Option<NaiveDate>, Option<NaiveDate>), String> {
        let row = sqlx::query(
            r#"
            SELECT MIN(ramp_date) as min_date, MAX(ramp_date) as max_date
            FROM fiat_ramp
            "#,
        )
        .fetch_one(&db.0)
        .await
        .map_err(|e| format!("failed to get date range: {e}"))?;

        use sqlx::Row;
        let min_date: Option<NaiveDate> = row.try_get("min_date").unwrap_or(None);
        let max_date: Option<NaiveDate> = row.try_get("max_date").unwrap_or(None);

        Ok((min_date, max_date))
    }

    /// Update the fiat ramp
    /// - returns the number of rows affected
    pub async fn update(update_ramp: UpdateFiatRamp, db: &Db) -> Result<u64, String> {
        let mut tx =
            db.0.begin()
                .await
                .map_err(|e| format!("failed to begin transaction: {e}"))?;

        let result: SqliteQueryResult = sqlx::query(
            r#"
            UPDATE fiat_ramp
            SET
            fiat_id = ?,
            fiat_amount = ?,
            ramp_date = ?,
            via_exchange = ?,
            kind = ?,
            updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        "#,
        )
        .bind(update_ramp.fiat_id)
        .bind(update_ramp.fiat_amount)
        .bind(update_ramp.ramp_date)
        .bind(update_ramp.via_exchange)
        .bind(update_ramp.kind)
        .bind(update_ramp.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("failed to update fiat_ramp table: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| format!("failed to commit transaction: {e}"))?;
        Ok(result.rows_affected())
    }

    /// Delete the fiat ramp
    /// - returns the number of rows affected
    pub async fn delete(id: StringRowId, db: &Db) -> Result<u64, String> {
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
    use std::str::FromStr;

    use super::*;
    use crate::fiat::FiatService;
    use crate::fiat_exchanger::Currency;
    use crate::fiat_exchanger::MockFiatExchanger;
    use rand::prelude::*;
    use sqlx::sqlite::SqlitePool;

    async fn init_db() -> Db {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let db = Db(pool);

        // create fiat table
        sqlx::migrate!("./migrations")
            .run(&db.0)
            .await
            .map_err(|e| format!("failed to run migrations: {e}"))
            .unwrap();

        let mut mock_api = MockFiatExchanger::new();
        mock_api.expect_get_available_currencies().return_once(|| {
            Ok(vec![
                Currency {
                    name: "Malaysian Ringgit".to_string(),
                    symbol: "MYR".to_string(),
                },
                Currency {
                    name: "Singapore Dollar".to_string(),
                    symbol: "SGD".to_string(),
                },
            ])
        });

        let fiat_service = FiatService::new(mock_api);
        fiat_service.update_currencies(&db).await.unwrap();

        // Setup user settings
        let fiat_id = sqlx::query_scalar::<_, i64>("SELECT id FROM fiat LIMIT 1")
            .fetch_one(&db.0)
            .await
            .unwrap();

        sqlx::query("INSERT OR REPLACE INTO user_settings (id, locale, default_fiat_id) VALUES (1, 'en', ?)")
            .bind(fiat_id)
            .execute(&db.0)
            .await
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
        let result = FiatRampService::create(create_fiat_ramp, &db).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_fiat_ramp() {
        let db = init_db().await;
        //create a fiat ramp 2 mock data
        let mut rng = rand::rng();
        for _ in 1..=11 {
            let fiat_id: i64 = rng.random_range(1..=2);
            let create_fiat_ramp = CreateFiatRamp {
                fiat_id,
                fiat_amount: 100.0,
                ramp_date: chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                via_exchange: "coinbase".to_string(),
                kind: RampKind::Deposit,
            };
            let _ = FiatRampService::create(create_fiat_ramp, &db).await;
        }

        let result = FiatRampService::get(10, 0, None, None, None, None, &db)
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

        let result = FiatRampService::get(10, 10, None, None, None, None, &db)
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

        assert!(!result.fiat_ramps[0].from_fiat_symbol.is_empty());
        let symbol = &result.fiat_ramps[0].from_fiat_symbol;
        assert!(matches!(symbol.as_str(), "MYR" | "SGD"));
    }

    #[tokio::test]
    async fn test_get_fiat_ramp_search() {
        let db = init_db().await;
        // create a fiat ramp
        let create_fiat_ramp = CreateFiatRamp {
            fiat_id: 1, // Depending on init_db order, this is either MYR or SGD
            fiat_amount: 100.0,
            ramp_date: chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            via_exchange: "coinbase".to_string(),
            kind: RampKind::Deposit,
        };
        let _ = FiatRampService::create(create_fiat_ramp, &db).await;

        // Determine which currency ID 1 is
        let fiat_name: String = sqlx::query_scalar("SELECT name FROM fiat WHERE id = 1")
            .fetch_one(&db.0)
            .await
            .unwrap();

        // Search by part of the name
        let part_of_name = &fiat_name[0..4]; // e.g., "Mala" or "Sing"
        let result =
            FiatRampService::get(10, 0, Some(part_of_name.to_string()), None, None, None, &db)
                .await
                .unwrap();

        assert_eq!(result.total_count, 1);
        assert_eq!(result.fiat_ramps.len(), 1);
        assert_eq!(result.fiat_ramps[0].from_fiat_name, fiat_name);

        // Search by something that doesn't exist
        let result = FiatRampService::get(
            10,
            0,
            Some("NonExistent".to_string()),
            None,
            None,
            None,
            &db,
        )
        .await
        .unwrap();
        assert_eq!(result.total_count, 0);
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
        let id = FiatRampService::create(create_fiat_ramp, &db)
            .await
            .unwrap();
        // assert id is kind of uuid v7
        assert!(Uuid::from_str(&id).is_ok());

        let update_ramp = UpdateFiatRamp {
            id,
            fiat_id: Some(1),
            fiat_amount: Some(200.0),
            ramp_date: Some(chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()),
            via_exchange: Some("coinbase".to_string()),
            kind: Some(RampKind::Deposit),
        };
        let result = FiatRampService::update(update_ramp, &db).await;
        assert!(result.unwrap() == 1);

        // check if the update was successful
        let result = FiatRampService::get(1, 0, None, None, None, None, &db).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.fiat_ramps.len() == 1);
        assert!(result.fiat_ramps[0].fiat_amount == 200.0);
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
        let id = FiatRampService::create(create_fiat_ramp, &db)
            .await
            .unwrap();
        // assert id is kind of uuid v7
        assert!(Uuid::from_str(&id).is_ok());

        let result = FiatRampService::delete(id, &db).await;
        assert!(result.unwrap() == 1);
    }

    #[tokio::test]
    async fn test_fiat_ramp_view() {
        let db = init_db().await;
        // 1. Create a User Settings with default Fiat as USD (id=1 is likely USD based on init_db mocks usually, but let's check or just use 1)
        // In `init_db`, migrations are run. `1_init.sql` likely creates fiat table. `3_create_fiat.sql` populates it?
        // Let's assume id 1 exists. In `test_create_fiat_ramp` we use fiat_id 1.

        // Insert User Settings (since init_db doesn't seem to do it explicitly, but `ensure_exists` does)
        // Let's manually insert user settings using SQL to be sure.
        sqlx::query("INSERT OR REPLACE INTO user_settings (id, locale, default_fiat_id) VALUES (1, 'en', 1)")
            .execute(&db.0)
            .await
            .unwrap();

        // 2. Create a Fiat Ramp
        // Let's say we dealt in EUR (which we need to make sure exists or just use ID 2)
        // In `init_db` mock api returns MYR and SGD. `update_currencies` populates them.
        // Let's check `init_db` in `fiat_ramp/mod.rs`:
        // It calls `FiatService::new(mock_api).update_currencies(&db)`.
        // Mock returns MYR and SGD.
        // So ID 1 might be MYR or SGD depending on insertion order.
        // Let's just use explicit SQL to insert fiats we want to test with to be safe, or query them.
        let usd_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO fiat (symbol, name) VALUES ('USD', 'US Dollar') RETURNING id",
        )
        .fetch_one(&db.0)
        .await
        .unwrap();
        let eur_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO fiat (symbol, name) VALUES ('EUR', 'Euro') RETURNING id",
        )
        .fetch_one(&db.0)
        .await
        .unwrap();

        // Set User Default to USD
        sqlx::query("UPDATE user_settings SET default_fiat_id = ? WHERE id = 1")
            .bind(usd_id)
            .execute(&db.0)
            .await
            .unwrap();

        let date = chrono::NaiveDate::from_ymd_opt(2023, 10, 25).unwrap();

        // Create Ramp in EUR
        let create_ramp = CreateFiatRamp {
            fiat_id: eur_id,
            fiat_amount: 100.0,
            ramp_date: date,
            via_exchange: "test".to_string(),
            kind: RampKind::Deposit,
        };
        FiatRampService::create(create_ramp, &db).await.unwrap();

        // 3. Insert Rates
        // We need a rate for EUR -> USD on that date.
        // Rate table has base_fiat_id.
        // If base is EUR, rates JSON should contain USD.
        let rates = serde_json::json!({
            "USD": 1.05,
            "EUR": 1.0
        });
        sqlx::query("INSERT INTO fiat_exchange_rate (base_fiat_id, date, rates) VALUES (?, ?, ?)")
            .bind(eur_id)
            .bind(date)
            .bind(rates.to_string())
            .execute(&db.0)
            .await
            .unwrap();

        // 4. Query View
        let view_result = sqlx::query_as::<sqlx::Sqlite, FiatRampWithConversionView>(
            "SELECT * FROM fiat_ramp_view WHERE kind = 'deposit'",
        )
        .fetch_one(&db.0)
        .await
        .unwrap();

        assert_eq!(view_result.to_fiat_symbol, "USD");

        // 5. Test Same Currency Conversion (Identity Case)
        // Create Ramp in USD (Default is USD)
        // No rate entry needed for this date/pair ideally if our view logic is correct.
        let create_ramp_usd = CreateFiatRamp {
            fiat_id: usd_id,
            fiat_amount: 50.0,
            ramp_date: date,
            via_exchange: "withdraw_test".to_string(),
            kind: RampKind::Withdraw,
        };
        FiatRampService::create(create_ramp_usd, &db).await.unwrap();

        let view_result_usd = sqlx::query_as::<sqlx::Sqlite, FiatRampWithConversionView>(
            "SELECT * FROM fiat_ramp_view WHERE kind = 'withdraw'",
        )
        .fetch_one(&db.0)
        .await
        .unwrap();

        assert_eq!(view_result_usd.to_fiat_symbol, "USD");
        assert_eq!(view_result_usd.from_fiat_symbol, "USD");
        // Conversion rate should be 1.0
        assert_eq!(view_result_usd.conversion_rate, Some(1.0));
        // Amount should be same
        assert_eq!(view_result_usd.converted_amount, Some(50.0));
    }

    #[tokio::test]
    async fn test_fiat_ramp_view_estimated() {
        let db = init_db().await;

        // 1. Setup User Settings (Default USD)
        let usd_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO fiat (symbol, name) VALUES ('USD', 'US Dollar') RETURNING id",
        )
        .fetch_one(&db.0)
        .await
        .unwrap();
        let eur_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO fiat (symbol, name) VALUES ('EUR', 'Euro') RETURNING id",
        )
        .fetch_one(&db.0)
        .await
        .unwrap();
        sqlx::query("INSERT OR REPLACE INTO user_settings (id, locale, default_fiat_id) VALUES (1, 'en', ?)")
            .bind(usd_id)
            .execute(&db.0).await.unwrap();

        let date = chrono::NaiveDate::from_ymd_opt(2023, 10, 26).unwrap();

        // 2. Create Ramp
        let create_ramp = CreateFiatRamp {
            fiat_id: eur_id,
            fiat_amount: 100.0,
            ramp_date: date,
            via_exchange: "est_test".to_string(),
            kind: RampKind::Deposit,
        };
        FiatRampService::create(create_ramp, &db).await.unwrap();

        // 3. Insert Estimated Rate (is_estimated = 1)
        let rates = serde_json::json!({ "USD": 1.10, "EUR": 1.0 });
        sqlx::query("INSERT INTO fiat_exchange_rate (base_fiat_id, date, rates, is_estimated) VALUES (?, ?, ?, ?)")
            .bind(eur_id)
            .bind(date)
            .bind(rates.to_string())
            .bind(true)
            .execute(&db.0).await.unwrap();

        // 4. Verify View
        let view_result = sqlx::query_as::<sqlx::Sqlite, FiatRampWithConversionView>(
            "SELECT * FROM fiat_ramp_view WHERE via_exchange = 'est_test'",
        )
        .fetch_one(&db.0)
        .await
        .unwrap();

        assert_eq!(view_result.is_estimated, true);
    }
}
