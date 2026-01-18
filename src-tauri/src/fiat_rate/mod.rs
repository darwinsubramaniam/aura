pub mod command;
use crate::db::StringRowId;
use crate::fiat::FiatService;
use crate::fiat_exchanger::frankfurter_exchanger::FrankfurterExchangerApi;
use crate::fiat_exchanger::Rates;
use crate::{db::Db, fiat_exchanger::FiatExchanger};
use anyhow::{Context, Result};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
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
    #[sqlx(default)]
    pub is_non_working_day: bool,
    #[sqlx(default)]
    pub non_working_day_reason: Option<String>,
    #[sqlx(json)]
    pub rates: HashMap<String, f64>,
}

#[derive(Debug, sqlx::FromRow)]
struct MissingRate {
    base_fiat_id: i64,
    date: NaiveDate,
}

const MAX_RETRIES: i32 = 5;

fn is_weekend(date: &NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

fn previous_business_day(date: &NaiveDate) -> NaiveDate {
    let mut candidate = *date - Duration::days(1);
    while is_weekend(&candidate) {
        candidate -= Duration::days(1);
    }
    candidate
}

// Get the rate for a specific fiat and date
// In FiatExchangeRate Table the base_fiat_id is the id of the USD dollar
pub async fn get_rate<A: FiatExchanger>(
    db: &Db,
    exchange_api: &A,
    date: &NaiveDate,
    fiat_ramp_id: Option<&StringRowId>,
) -> Result<FiatExchangeRate> {
    let usd_fiat = FiatService::<FrankfurterExchangerApi>::get_fiat_by_symbol(db, "USD").await?;
    // 1. Check DB first
    let result = sqlx::query_as::<sqlx::Sqlite, FiatExchangeRate>(
        r#"
        SELECT
        fiat_exchange_rate.*,
        fiat.symbol,
        fiat.name
        FROM fiat_exchange_rate
        JOIN fiat ON fiat_exchange_rate.base_fiat_id = fiat.id
        WHERE fiat_exchange_rate.base_fiat_id = ?
        AND fiat_exchange_rate.date = ?
        "#,
    )
    .bind(&usd_fiat.id)
    .bind(date)
    .fetch_one(&db.0)
    .await;

    if let Ok(rate) = result {
        // Rate Exists
        // If exact match (not estimated), and we have a ramp_id, ensure we are NOT in the queue.
        // Actually, even if estimated, if we found it in DB, we use it.
        // But the user might be calling this to trigger a "re-check" or "ensure consistent".

        // If we have a ramp_id, update the queue only when it's a true retry case.
        // Estimated rates caused by non-working days (weekends/holidays) should NOT be queued.
        if let Some(id) = fiat_ramp_id {
            if !rate.is_estimated || rate.is_non_working_day {
                remove_from_missing_queue_by_ramp_id(db, id).await.ok();
            } else {
                add_to_missing_queue(db, id, usd_fiat.id, date, Some("Using estimated rate"))
                    .await
                    .ok();
            }
        }

        return Ok(rate);
    }

    // 2. Not found in DB, fetch from API
    let usd_fiat = FiatService::<A>::get_fiat_by_id(db, usd_fiat.id)
        .await
        .context("failed to get fiat by id")?;
    let base_symbol = usd_fiat.symbol;
    let base_name = usd_fiat.name;

    // Call API
    let api_result = exchange_api
        .get_latest_rates(&base_symbol.as_str(), Some(date))
        .await;

    match api_result {
        Ok(api_rates) => {
            let (mut fiat_rate, should_retry) = process_and_insert_rate(
                db,
                usd_fiat.id,
                date,
                api_rates,
                &base_symbol,
            )
            .await?;

            if should_retry {
                if let Some(id) = fiat_ramp_id {
                    add_to_missing_queue(
                        db,
                        id,
                        usd_fiat.id,
                        date,
                        Some("Fallback: 1 day difference"),
                    )
                    .await?;
                }
            } else {
                remove_from_missing_queue_by_rate(db, usd_fiat.id, date)
                    .await
                    .ok();
                if let Some(id) = fiat_ramp_id {
                    remove_from_missing_queue_by_ramp_id(db, id).await.ok();
                }
            }

            fiat_rate.symbol = base_symbol;
            fiat_rate.name = base_name;
            Ok(fiat_rate)
        }
        Err(e) => {
            // API Failure
            if let Some(id) = fiat_ramp_id {
                add_to_missing_queue(db, id, usd_fiat.id, date, Some(&e.to_string())).await?;
            }
            Err(e.into())
        }
    }
}


pub async fn process_missing_rates<A: FiatExchanger>(db: &Db, exchange_api: &A) -> Result<()> {
    // 1. Fetch unique missing rates (grouped by fiat/date)
    // We only care about distinct rates that are missing.
    let missing_items = sqlx::query_as::<sqlx::Sqlite, MissingRate>(
        r#"
        SELECT
        DISTINCT base_fiat_id, date
        FROM fiat_rate_missing
        WHERE error_count < ?
        "#,
    )
    .bind(MAX_RETRIES)
    .fetch_all(&db.0)
    .await
    .context("Failed to fetch missing rates queue")?;

    for item in missing_items {
        // We use get_fiat_by_id to get symbol
        if let Ok(base_fiat) = FiatService::<A>::get_fiat_by_id(db, item.base_fiat_id).await {
            match exchange_api
                .get_latest_rates(&base_fiat.symbol, Some(&item.date))
                .await
            {
                Ok(api_rates) => {
                    match process_and_insert_rate(
                        db,
                        item.base_fiat_id,
                        &item.date,
                        api_rates,
                        &base_fiat.symbol,
                    )
                    .await
                    {
                        Ok((_, should_retry)) => {
                            if !should_retry {
                                remove_from_missing_queue_by_rate(db, item.base_fiat_id, &item.date)
                                    .await
                                    .ok();
                            } else {
                                // Inserted fallback, but keep in queue
                                update_error_count_by_rate(
                                    db,
                                    item.base_fiat_id,
                                    &item.date,
                                    "Fallback inserted: 1 day diff",
                                )
                                .await
                                .ok();
                            }
                        }
                        Err(e) => {
                            update_error_count_by_rate(
                                db,
                                item.base_fiat_id,
                                &item.date,
                                &e.to_string(),
                            )
                            .await
                            .ok();
                        }
                    }
                }
                Err(e) => {
                    update_error_count_by_rate(db, item.base_fiat_id, &item.date, &e.to_string())
                        .await
                        .ok();
                }
            }
        }
    }
    Ok(())
}

/// Helper: Process and insert rate (shared logic)
/// - Returns: Rate and if the re-fetch should be retried, as exchanger might still not yet in the date zone of the user.
async fn process_and_insert_rate(
    db: &Db,
    base_fiat_id: i64,
    target_date: &NaiveDate,
    mut api_rates: Rates,
    base_symbol: &str,
) -> Result<(FiatExchangeRate, bool)> {
    let diff = (*target_date - api_rates.date).num_days();

    // Insert base rate = 1.0 (Frankfurter API logic)
    api_rates.rates.insert(base_symbol.to_string(), 1.0);

    let is_non_working_day = diff > 0 && api_rates.date == previous_business_day(target_date);
    let non_working_day_reason = if is_non_working_day {
        Some(if is_weekend(target_date) {
            "weekend"
        } else {
            "public_holiday"
        })
    } else {
        None
    };

    let (is_estimated, is_nwd, final_reason, should_retry) =
        determine_rate_status(diff, is_non_working_day, non_working_day_reason);

    let fiat_rate = insert_rate(
        db,
        base_fiat_id,
        target_date,
        &api_rates.rates,
        is_estimated,
        is_nwd,
        final_reason,
    )
    .await?;

    Ok((fiat_rate, should_retry))
}

// Helper: Determine rate status based on diff and non-working day
// Returns: (is_estimated, is_non_working_day, final_reason, should_retry)
// should_retry = true means we should keep attempting to fetch better data (add/keep in queue)
// should_retry = false means we have a definitive result (remove from queue)
fn determine_rate_status(
    diff: i64,
    is_non_working_day: bool,
    non_working_day_reason: Option<&'static str>,
) -> (bool, bool, Option<&'static str>, bool) {
    if diff == 0 {
        (false, false, None, false)
    } else if is_non_working_day {
        (true, true, non_working_day_reason, false)
    } else if diff == 1 {
        // Fallback: 1 day old
        // Estimated, but we want to retry (should_retry = true)
        (true, false, None, true)
    } else {
        // > 1 day: Exchange closed
        // Estimated, do NOT retry (should_retry = false)
        (true, true, Some("exchange closed"), false)
    }
}

// Helper: Insert (or Replace) Rate
async fn insert_rate(
    db: &Db,
    base_fiat_id: i64,
    date: &NaiveDate,
    rates: &HashMap<String, f64>,
    is_estimated: bool,
    is_non_working_day: bool,
    non_working_day_reason: Option<&str>,
) -> Result<FiatExchangeRate> {
    // We use INSERT OR REPLACE to handle cases where we are upgrading an estimated rate to a real one
    let sql = r#"
        INSERT OR REPLACE
        INTO fiat_exchange_rate
        (base_fiat_id, date, rates, is_estimated, is_non_working_day, non_working_day_reason)
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *
    "#
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    let fiat_rate = sqlx::query_as::<sqlx::Sqlite, FiatExchangeRate>(&sql)
        .bind(base_fiat_id)
        .bind(date)
        .bind(sqlx::types::Json(rates))
        .bind(is_estimated)
        .bind(is_non_working_day)
        .bind(non_working_day_reason)
        .fetch_one(&db.0)
        .await
        .context("Failed to insert/replace rate")?;
    Ok(fiat_rate)
}

// Helper: Add to Missing Queue (Upsert based on Ramp ID)
// - If fiat_ramp_id exists with same base_fiat_id AND date: only increment error_count
// - If fiat_ramp_id exists with different base_fiat_id OR date: insert new values and reset error_count
// - If fiat_ramp_id doesn't exist: insert new record
async fn add_to_missing_queue(
    db: &Db,
    fiat_ramp_id: &str,
    base_fiat_id: i64,
    date: &NaiveDate,
    error_msg: Option<&str>,
) -> Result<()> {
    sqlx::query(
        "
        INSERT INTO fiat_rate_missing (fiat_ramp_id, base_fiat_id, date, error_count, last_error_msg, last_attempt_at)
        VALUES (?, ?, ?, 1, ?, CURRENT_TIMESTAMP)
        ON CONFLICT(fiat_ramp_id) DO UPDATE SET
            base_fiat_id = CASE
                WHEN fiat_rate_missing.base_fiat_id = excluded.base_fiat_id AND fiat_rate_missing.date = excluded.date
                THEN fiat_rate_missing.base_fiat_id
                ELSE excluded.base_fiat_id
            END,
            date = CASE
                WHEN fiat_rate_missing.base_fiat_id = excluded.base_fiat_id AND fiat_rate_missing.date = excluded.date
                THEN fiat_rate_missing.date
                ELSE excluded.date
            END,
            error_count = CASE
                WHEN fiat_rate_missing.base_fiat_id = excluded.base_fiat_id AND fiat_rate_missing.date = excluded.date
                THEN fiat_rate_missing.error_count + 1
                ELSE 1
            END,
            last_error_msg = excluded.last_error_msg,
            last_attempt_at = CURRENT_TIMESTAMP
        ",
    )
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
async fn remove_from_missing_queue_by_ramp_id(db: &Db, fiat_ramp_id: &str) -> Result<()> {
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
    date: &NaiveDate,
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
    error_msg: &str,
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
            &NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
            None,
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

        // Requested: 2026-02-07 (Saturday)
        // API Returns: 2026-02-06 (Friday)
        mock_frankfurt_api
            .expect_get_latest_rates()
            .times(1)
            .returning(|_, _| {
                Ok(Rates {
                    rates: HashMap::from([("EUR".to_string(), 0.85)]),
                    base: "USD".to_string(),
                    date: NaiveDate::from_ymd_opt(2026, 2, 6).unwrap(),
                })
            });

        let request_date = NaiveDate::from_ymd_opt(2026, 2, 7).unwrap();

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
            &request_date,
            Some(&ramp_id.to_string()),
        )
        .await;

        assert!(rate.is_ok());
        let rate = rate.unwrap();

        // Should be estimated and marked as a non-working day
        assert!(rate.is_estimated);
        assert!(rate.is_non_working_day);
        assert_eq!(rate.non_working_day_reason.as_deref(), Some("weekend"));
        // Date in DB should be requested date
        assert_eq!(rate.date, request_date);

        // Check queue (non-working days should NOT be queued)
        let queue: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM fiat_rate_missing")
            .fetch_one(&db.0)
            .await
            .unwrap();
        assert_eq!(queue, 0);
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
            &request_date,
            Some(&ramp_id.to_string()),
        )
        .await;

        assert!(rate.is_ok());
        let rate = rate.unwrap();

        assert!(rate.is_estimated);
        assert!(rate.is_non_working_day);
        assert_eq!(rate.non_working_day_reason.as_deref(), Some("exchange closed"));

        // Check queue (should be empty as we treat it as closed and don't retry)
        let queue: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM fiat_rate_missing")
            .fetch_one(&db.0)
            .await
            .unwrap();
        assert_eq!(queue, 0);
    }

    #[tokio::test]
    async fn test_process_missing_rates() {
        let db = setup().await;
        let mut mock_api = MockFiatExchanger::new();

        let date = NaiveDate::from_ymd_opt(2026, 2, 2).unwrap();
        let exact_date = NaiveDate::from_ymd_opt(2026, 2, 2).unwrap();

        // 1. Manually insert a missing item
        let eur_id = sqlx::query_scalar::<_, i64>("SELECT id FROM fiat WHERE symbol='EUR'")
            .fetch_one(&db.0)
            .await
            .unwrap();

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
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM fiat_rate_missing")
            .fetch_one(&db.0)
            .await
            .unwrap();
        assert_eq!(count, 0);

        // 5. Verify rate exists and is NOT estimated
        let rate = sqlx::query_as::<sqlx::Sqlite, FiatExchangeRate>(
            "SELECT * FROM fiat_exchange_rate WHERE base_fiat_id = ? AND date = ?",
        )
        .bind(eur_id)
        .bind(date)
        .fetch_one(&db.0)
        .await
        .unwrap();

        assert!(!rate.is_estimated);
    }
}
