pub mod command;
use crate::db::StringRowId;
use crate::fiat::FiatService;
use crate::{db::Db, fiat_exchanger::FiatExchanger};
use anyhow::{Context, Result};
use chrono::NaiveDate;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct FiatExchangeRate {
    pub id: i64,
    pub base_fiat_id: i64,
    #[sqlx(skip)]
    pub symbol: String,
    #[sqlx(skip)]
    pub name: String,
    pub date: NaiveDate,
    #[sqlx(default)]
    pub is_estimated: bool,
    #[sqlx(json)]
    pub rates: HashMap<String, f64>,
}

#[derive(Debug, sqlx::FromRow)]
struct MissingRate {
    base_fiat_id: i64,
    date: NaiveDate,
}

const MAX_RETRIES: i32 = 5;

// Get the rate for a specific fiat and date
pub async fn get_rate<A: FiatExchanger>(
    db: &Db,
    exchange_api: &A,
    base_fiat_id: i64,
    date: &NaiveDate,
    fiat_ramp_id: Option<&StringRowId>,
) -> Result<FiatExchangeRate> {
    // 1. Check DB first
    let db_result = sqlx::query_as::<sqlx::Sqlite, FiatExchangeRate>(
        "SELECT fiat_exchange_rate.*, fiat.symbol, fiat.name FROM fiat_exchange_rate 
        JOIN fiat ON fiat_exchange_rate.base_fiat_id = fiat.id 
        WHERE fiat_exchange_rate.base_fiat_id = ? AND fiat_exchange_rate.date = ?",
    )
    .bind(base_fiat_id)
    .bind(date)
    .fetch_one(&db.0)
    .await;

    if let Ok(rate) = db_result {
        // Rate Exists
        // If exact match (not estimated), and we have a ramp_id, ensure we are NOT in the queue.
        // Actually, even if estimated, if we found it in DB, we use it.
        // But the user might be calling this to trigger a "re-check" or "ensure consistent".
        
        // If we have a ramp_id, we should update the queue to reflect that this ramp is waiting for THIS date.
        // But if the rate is already good (not estimated), we should remove from queue.
        if !rate.is_estimated {
            if let Some(id) = fiat_ramp_id {
                remove_from_missing_queue_by_ramp_id(db, id).await.ok();
            }
        } else {
             // Rate is estimated.
             // If ramp_id provided, ensure it's in the queue for THIS date/fiat (handling the update case).
             if let Some(id) = fiat_ramp_id {
                 add_to_missing_queue(db, id, base_fiat_id, date, Some("Using estimated rate")).await.ok();
             }
        }
        
        return Ok(rate);
    }

    // 2. Not found in DB, fetch from API
    let base_fiat = FiatService::<A>::get_fiat_by_id(db, base_fiat_id)
        .await
        .context("failed to get fiat by id")?;
    let base_symbol = base_fiat.symbol;
    let base_name = base_fiat.name;

    // Call API
    let api_result = exchange_api
        .get_latest_rates(&base_symbol.as_str(), Some(date))
        .await;

    match api_result {
        Ok(mut api_rates) => {
            let diff = (*date - api_rates.date).num_days();

            // Insert base rate = 1.0 (Frankfurter API logic)
            api_rates.rates.insert(base_symbol.clone(), 1.0);

            if diff == 0 {
                // Exact match
                let fiat_rate = insert_rate(db, base_fiat_id, date, &api_rates.rates, false).await?;
                
                // Success - remove from missing queue for ALL ramps waiting for this rate
                remove_from_missing_queue_by_rate(db, base_fiat_id, date).await.ok();
                // Also explicitly remove for this ramp_id (just in case it was waiting for a diff date before)
                if let Some(id) = fiat_ramp_id {
                     remove_from_missing_queue_by_ramp_id(db, id).await.ok();
                }

                Ok(FiatExchangeRate {
                    id: fiat_rate.id,
                    base_fiat_id,
                    symbol: base_symbol,
                    name: base_name,
                    date: *date,
                    is_estimated: false,
                    rates: api_rates.rates,
                })
            } else if diff == 1 {
                // Fallback (1 day old)
                // Insert into DB as *estimated*, storing it under the requested `date`
                let fiat_rate = insert_rate(db, base_fiat_id, date, &api_rates.rates, true).await?;

                // Add to missing queue to retry later for exact data
                if let Some(id) = fiat_ramp_id {
                    add_to_missing_queue(db, id, base_fiat_id, date, Some("Fallback: 1 day difference")).await?;
                }

                Ok(FiatExchangeRate {
                    id: fiat_rate.id,
                    base_fiat_id,
                    symbol: base_symbol,
                    name: base_name,
                    date: *date,
                    is_estimated: true,
                    rates: api_rates.rates,
                })
            } else {
                // Too old (> 1 day)
                let msg = format!("Rate too old. Gap: {} days", diff);
                if let Some(id) = fiat_ramp_id {
                    add_to_missing_queue(db, id, base_fiat_id, date, Some(&msg)).await?;
                }
                Err(anyhow::anyhow!(msg))
            }
        },
        Err(e) => {
            // API Failure
            if let Some(id) = fiat_ramp_id {
                add_to_missing_queue(db, id, base_fiat_id, date, Some(&e.to_string())).await?;
            }
            Err(e.into())
        }
    }
}

pub async fn process_missing_rates<A: FiatExchanger>(db: &Db, exchange_api: &A) -> Result<()> {
    // 1. Fetch unique missing rates (grouped by fiat/date)
    // We only care about distinct rates that are missing.
    let missing_items = sqlx::query_as::<sqlx::Sqlite, MissingRate>(
        "SELECT DISTINCT base_fiat_id, date FROM fiat_rate_missing WHERE error_count < ?"
    )
    .bind(MAX_RETRIES)
    .fetch_all(&db.0)
    .await
    .context("Failed to fetch missing rates queue")?;

    for item in missing_items {
        // We use get_fiat_by_id to get symbol
        if let Ok(base_fiat) = FiatService::<A>::get_fiat_by_id(db, item.base_fiat_id).await {
            match exchange_api.get_latest_rates(&base_fiat.symbol, Some(&item.date)).await {
                Ok(mut api_rates) => {
                    let diff = (item.date - api_rates.date).num_days();
                    if diff == 0 {
                        // Found exact!
                        api_rates.rates.insert(base_fiat.symbol.clone(), 1.0);
                        
                        if let Ok(_) = insert_rate(db, item.base_fiat_id, &item.date, &api_rates.rates, false).await {
                             // Success! Remove ALL queue entries waiting for this rate
                             remove_from_missing_queue_by_rate(db, item.base_fiat_id, &item.date).await.ok();
                        }
                    } else {
                        // Still old. Just update error count for ALL ramps waiting for this.
                        update_error_count_by_rate(db, item.base_fiat_id, &item.date, "Retry: Still getting old data").await.ok();
                    }
                },
                Err(e) => {
                    update_error_count_by_rate(db, item.base_fiat_id, &item.date, &e.to_string()).await.ok();
                }
            }
        }
    }
    Ok(())
}

// Helper: Insert (or Replace) Rate
async fn insert_rate(db: &Db, base_fiat_id: i64, date: &NaiveDate, rates: &HashMap<String, f64>, is_estimated: bool) -> Result<FiatExchangeRate> {
    // We use INSERT OR REPLACE to handle cases where we are upgrading an estimated rate to a real one
    let fiat_rate = sqlx::query_as::<sqlx::Sqlite, FiatExchangeRate>(
        "INSERT OR REPLACE INTO fiat_exchange_rate (base_fiat_id, date, rates, is_estimated) VALUES (?, ?, ?, ?) RETURNING *"
    )
    .bind(base_fiat_id)
    .bind(date)
    .bind(sqlx::types::Json(rates))
    .bind(is_estimated)
    .fetch_one(&db.0)
    .await
    .context("Failed to insert/replace rate")?;
    Ok(fiat_rate)
}

// Helper: Add to Missing Queue (Upsert based on Ramp ID)
async fn add_to_missing_queue(
    db: &Db,
    fiat_ramp_id: &str,
    base_fiat_id: i64,
    date: &NaiveDate,
    error_msg: Option<&str>
) -> Result<()> {
    // If we are adding/updating for a ramp, we update the DATE and FIAT to the new requirements.
    // And reset error count if the requirements changed (handled by logic above? No, we should reset if we are upserting a new requirement).
    // Actually, simple upsert:
    // If ID exists -> Update to new Date/Fiat.
    // If not -> Insert.
    sqlx::query("
        INSERT INTO fiat_rate_missing (fiat_ramp_id, base_fiat_id, date, error_count, last_error_msg, last_attempt_at)
        VALUES (?, ?, ?, 1, ?, CURRENT_TIMESTAMP)
        ON CONFLICT(fiat_ramp_id) DO UPDATE SET
            base_fiat_id = excluded.base_fiat_id,
            date = excluded.date,
            error_count = CASE 
                WHEN fiat_rate_missing.base_fiat_id = excluded.base_fiat_id AND fiat_rate_missing.date = excluded.date THEN fiat_rate_missing.error_count + 1
                ELSE 1 
            END,
            last_error_msg = excluded.last_error_msg,
            last_attempt_at = CURRENT_TIMESTAMP
    ")
    .bind(fiat_ramp_id)
    .bind(base_fiat_id)
    .bind(date)
    .bind(error_msg)
    .execute(&db.0)
    .await
    .context("Failed to add to missing queue")?;
    Ok(())
}

// Remove by Ramp ID
async fn remove_from_missing_queue_by_ramp_id(
    db: &Db,
    fiat_ramp_id: &str
) -> Result<()> {
    sqlx::query("DELETE FROM fiat_rate_missing WHERE fiat_ramp_id = ?")
        .bind(fiat_ramp_id)
        .execute(&db.0)
        .await
        .context("Failed to remove from missing queue")?;
    Ok(())
}

// Remove by Rate (Clear all ramps waiting for this rate)
async fn remove_from_missing_queue_by_rate(
    db: &Db,
    base_fiat_id: i64,
    date: &NaiveDate
) -> Result<()> {
    sqlx::query("DELETE FROM fiat_rate_missing WHERE base_fiat_id = ? AND date = ?")
        .bind(base_fiat_id)
        .bind(date)
        .execute(&db.0)
        .await
        .context("Failed to remove from missing queue by rate")?;
    Ok(())
}

async fn update_error_count_by_rate(
    db: &Db,
    base_fiat_id: i64,
    date: &NaiveDate,
    error_msg: &str
) -> Result<()> {
     sqlx::query("UPDATE fiat_rate_missing SET error_count = error_count + 1, last_error_msg = ?, last_attempt_at = CURRENT_TIMESTAMP WHERE base_fiat_id = ? AND date = ?")
        .bind(error_msg)
        .bind(base_fiat_id)
        .bind(date)
        .execute(&db.0)
        .await
        .context("Failed to update error count")?;
    Ok(())
}

pub async fn get_conversion_amount(
    rates: HashMap<String, f64>,
    amount: f64,
    to: &str,
    from: &str,
) -> Result<f64> {
    let to_rate = rates
        .get(to)
        .context(format!("failed to get TO rate: {}", to))?;
    let from_rate = rates
        .get(from)
        .context(format!("failed to get FROM rate: {}", from))?;

    let to_rate_dec = Decimal::from_f64_retain(*to_rate).context("invalid to_rate")?;
    let from_rate_dec = Decimal::from_f64_retain(*from_rate).context("invalid from_rate")?;
    let amount_dec = Decimal::from_f64_retain(amount).context("invalid amount")?;

    let converted_amount = amount_dec * (to_rate_dec / from_rate_dec);
    let rounded_value = converted_amount.round_dp(2);
    
    Ok(rounded_value.to_f64().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::Db,
        fiat_exchanger::{Currency, MockFiatExchanger, Rates},
    };
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
        // create fiat_exchange_rate table
        sqlx::migrate!("./migrations")
            .run(&db.0)
            .await
            .context("failed to create fiat_exchange_rate table")
            .unwrap();

        // Need to create fiat_ramp table for FK constraint?
        // Migration 13 adds FK to fiat_ramp.
        // But our migrations include creating fiat_ramp (5_create_table_fiat_ramp.sql).
        // So it should be fine.

        // Mock API
        let mut mock_api = MockFiatExchanger::new();
        mock_api
            .expect_get_available_currencies()
            .times(1)
            .returning(|| {
                Ok(vec![
                    Currency {
                        name: "United States Dollar".to_string(),
                        symbol: "USD".to_string(),
                    },
                    Currency {
                        name: "Euro".to_string(),
                        symbol: "EUR".to_string(),
                    },
                ])
            });

        // populate fiat table
        FiatService::<MockFiatExchanger>::new(mock_api)
            .update_currencies(&db)
            .await
            .unwrap();

        db
    }

    #[tokio::test]
    async fn test_get_rate() {
        let db = setup().await;
        let mut mock_frankfurt_api = MockFiatExchanger::new();
        mock_frankfurt_api
            .expect_get_latest_rates()
            .times(1)
            .returning(|_, _| {
                Ok(Rates {
                    rates: HashMap::from([("EUR".to_string(), 0.85), ("MYR".to_string(), 4.7424)]),
                    base: "USD".to_string(),
                    date: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
                })
            });
        let rate = get_rate(
            &db,
            &mock_frankfurt_api,
            1,
            &NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
            None
        )
        .await;
        assert!(rate.is_ok());
        let rate = rate.unwrap();
        //ensure base fiat rate is 1
        assert_eq!(rate.rates.get("USD").unwrap(), &1.0);
        //ensure other fiat rates are correct
        assert_eq!(rate.rates.get("EUR").unwrap(), &0.85);
        assert_eq!(rate.rates.get("MYR").unwrap(), &4.7424);
        assert!(!rate.is_estimated);
    }

    #[tokio::test]
    async fn test_get_rate_fallback() {
        let db = setup().await;
        let mut mock_frankfurt_api = MockFiatExchanger::new();
        
        // Requested: 2026-02-02
        // API Returns: 2026-02-01 (1 day gap)
        mock_frankfurt_api
            .expect_get_latest_rates()
            .times(1)
            .returning(|_, _| {
                Ok(Rates {
                    rates: HashMap::from([("EUR".to_string(), 0.85)]),
                    base: "USD".to_string(),
                    date: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
                })
            });
            
        let request_date = NaiveDate::from_ymd_opt(2026, 2, 2).unwrap();
        
        // Need a ramp ID for queue testing
        // Insert a dummy ramp to satisfy FK
        let ramp_id = "test-ramp-id";
        sqlx::query("INSERT INTO fiat_ramp (id, fiat_id, fiat_amount, ramp_date, via_exchange, kind) VALUES (?, 1, 100, ?, 'test', 'deposit')")
            .bind(ramp_id)
            .bind(request_date)
            .execute(&db.0).await.unwrap();

        let rate = get_rate(
            &db,
            &mock_frankfurt_api,
            1,
            &request_date,
            Some(&ramp_id.to_string())
        )
        .await;
        
        assert!(rate.is_ok());
        let rate = rate.unwrap();
        
        // Should be estimated
        assert!(rate.is_estimated);
        // Date in DB should be requested date
        assert_eq!(rate.date, request_date);
        
        // Check queue
        let queue: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM fiat_rate_missing")
            .fetch_one(&db.0).await.unwrap();
        assert_eq!(queue, 1);
        
        let queued_ramp_id: String = sqlx::query_scalar("SELECT fiat_ramp_id FROM fiat_rate_missing LIMIT 1")
            .fetch_one(&db.0).await.unwrap();
        assert_eq!(queued_ramp_id, ramp_id);
    }
    
    #[tokio::test]
    async fn test_get_rate_too_old() {
        let db = setup().await;
        let mut mock_frankfurt_api = MockFiatExchanger::new();
        
        // Requested: 2026-02-03
        // API Returns: 2026-02-01 (2 days gap)
        mock_frankfurt_api
            .expect_get_latest_rates()
            .times(1)
            .returning(|_, _| {
                Ok(Rates {
                    rates: HashMap::from([("EUR".to_string(), 0.85)]),
                    base: "USD".to_string(),
                    date: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
                })
            });
            
        let request_date = NaiveDate::from_ymd_opt(2026, 2, 3).unwrap();
        
        let ramp_id = "test-ramp-old";
        sqlx::query("INSERT INTO fiat_ramp (id, fiat_id, fiat_amount, ramp_date, via_exchange, kind) VALUES (?, 1, 100, ?, 'test', 'deposit')")
            .bind(ramp_id)
            .bind(request_date)
            .execute(&db.0).await.unwrap();

        let rate = get_rate(
            &db,
            &mock_frankfurt_api,
            1,
            &request_date,
            Some(&ramp_id.to_string())
        )
        .await;
        
        assert!(rate.is_err());
        
        // Check queue
        let queue: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM fiat_rate_missing")
            .fetch_one(&db.0).await.unwrap();
        assert_eq!(queue, 1);
    }

    #[tokio::test]
    async fn test_process_missing_rates() {
        let db = setup().await;
        let mut mock_api = MockFiatExchanger::new();
        
        let date = NaiveDate::from_ymd_opt(2026, 2, 2).unwrap();
        let exact_date = NaiveDate::from_ymd_opt(2026, 2, 2).unwrap();
        
        // 1. Manually insert a missing item
        let eur_id = sqlx::query_scalar::<_, i64>("SELECT id FROM fiat WHERE symbol='EUR'").fetch_one(&db.0).await.unwrap();
        
        let ramp_id = "test-process-ramp";
        sqlx::query("INSERT INTO fiat_ramp (id, fiat_id, fiat_amount, ramp_date, via_exchange, kind) VALUES (?, ?, 100, ?, 'test', 'deposit')")
            .bind(ramp_id)
            .bind(eur_id)
            .bind(date)
            .execute(&db.0).await.unwrap();

        sqlx::query("INSERT INTO fiat_rate_missing (fiat_ramp_id, base_fiat_id, date, error_count) VALUES (?, ?, ?, 0)")
            .bind(ramp_id)
            .bind(eur_id)
            .bind(date)
            .execute(&db.0).await.unwrap();
            
        // 2. Mock API to return EXACT match now
        mock_api
            .expect_get_latest_rates()
            .times(1)
            .returning(move |_, _| {
                Ok(Rates {
                    rates: HashMap::from([("USD".to_string(), 1.10)]),
                    base: "EUR".to_string(),
                    date: exact_date,
                })
            });
            
        // 3. Process
        process_missing_rates(&db, &mock_api).await.unwrap();
        
        // 4. Verify queue is empty
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM fiat_rate_missing").fetch_one(&db.0).await.unwrap();
        assert_eq!(count, 0);
        
        // 5. Verify rate exists and is NOT estimated
        let rate = sqlx::query_as::<sqlx::Sqlite, FiatExchangeRate>(
            "SELECT * FROM fiat_exchange_rate WHERE base_fiat_id = ? AND date = ?"
        )
        .bind(eur_id)
        .bind(date)
        .fetch_one(&db.0).await.unwrap();
        
        assert!(!rate.is_estimated);
    }
}
