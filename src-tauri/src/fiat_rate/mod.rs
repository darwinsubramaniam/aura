pub mod command;
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
    #[sqlx(json)]
    pub rates: HashMap<String, f64>,
}

// Get the rate for a specific fiat and date
pub async fn get_rate<A: FiatExchanger>(
    db: &Db,
    exchange_api: &A,
    base_fiat_id: i64,
    date: &NaiveDate,
) -> Result<FiatExchangeRate> {
    let db_result = sqlx::query_as::<sqlx::Sqlite, FiatExchangeRate>(
        "SELECT fiat_exchange_rate.*, fiat.symbol, fiat.name FROM fiat_exchange_rate 
        JOIN fiat ON fiat_exchange_rate.base_fiat_id = fiat.id 
        WHERE fiat_exchange_rate.base_fiat_id = ? AND fiat_exchange_rate.date = ?",
    )
    .bind(base_fiat_id)
    .bind(date)
    .fetch_one(&db.0)
    .await;

    match db_result {
        Ok(rate) => Ok(rate),
        Err(sqlx::Error::RowNotFound) => {
            let base_fiat = FiatService::<A>::get_fiat_by_id(db, base_fiat_id)
                .await
                .context("failed to get fiat by id")?;
            let base_symbol = base_fiat.symbol;
            let base_name = base_fiat.name;
            // forward the request to the Frankfurter API
            let mut rate = exchange_api
                .get_latest_rates(&base_symbol.as_str(), Some(date))
                .await
                .context("External API :: Get Latest Rates :: Failed")?;
            // add the base fiat rate to the rates
            rate.rates.insert(base_symbol.clone(), 1.0);

            // insert the rate into the database
            let mut fiat_rate = sqlx::query_as::<sqlx::Sqlite, FiatExchangeRate>(
                "INSERT INTO fiat_exchange_rate (base_fiat_id, date, rates) VALUES (?, ?, ?) RETURNING *",
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
        )
        .await;
        assert!(rate.is_ok());
        let rate = rate.unwrap();
        //ensure base fiat rate is 1
        assert_eq!(rate.rates.get("USD").unwrap(), &1.0);
        //ensure other fiat rates are correct
        assert_eq!(rate.rates.get("EUR").unwrap(), &0.85);
        assert_eq!(rate.rates.get("MYR").unwrap(), &4.7424);
    }

    #[tokio::test]
    async fn test_get_conversion_amount() {
        let rates = HashMap::from([
            ("AUD".to_string(), 1.7441),
            ("BRL".to_string(), 6.2733),
            ("CAD".to_string(), 1.6163),
            ("CHF".to_string(), 0.9314),
            ("CNY".to_string(), 8.1288),
            ("CZK".to_string(), 24.337),
            ("DKK".to_string(), 7.4724),
            ("GBP".to_string(), 0.8677),
            ("HKD".to_string(), 9.0763),
            ("HUF".to_string(), 386.03),
            ("IDR".to_string(), 19628.24),
            ("ILS".to_string(), 3.6745),
            ("INR".to_string(), 105.0335),
            ("ISK".to_string(), 147.4),
            ("JPY".to_string(), 183.52),
            ("KRW".to_string(), 1699.55),
            ("MXN".to_string(), 20.9879),
            ("MYR".to_string(), 4.7424),
            ("NOK".to_string(), 11.7765),
            ("NZD".to_string(), 2.0339),
            ("PHP".to_string(), 68.935),
            ("PLN".to_string(), 4.2138),
            ("RON".to_string(), 5.0902),
            ("SEK".to_string(), 10.748),
            ("SGD".to_string(), 1.4984),
            ("THB".to_string(), 36.632),
            ("TRY".to_string(), 50.1841),
            ("USD".to_string(), 1.1642),
            ("ZAR".to_string(), 19.2966),
            ("EUR".to_string(), 1.0),
        ]);

        /*
        | From | To  | Amount | Expected (2 dp) |
        | ---- | --- | ------ | --------------- |
        | USD  | MYR | 123.45 | 502.88          |
        | SGD  | INR | 50.00  | 3504.86         |
        | JPY  | GBP | 2500.0 | 11.82           |
        | AUD  | USD | 75.25  | 50.23           |
        | KRW  | SGD | 999.99 | 0.88            |
        | EUR  | THB | 10.00  | 366.32          |
        | MYR  | USD | 500.00 | 122.74          |
        | CHF  | NOK | 33.33  | 421.42          |
        | PLN  | HKD | 88.80  | 191.27          |
        | ZAR  | CAD | 100.00 | 8.38            |

        */
        let cases = vec![
            ("USD", "MYR", 123.45, 502.88),
            ("SGD", "INR", 50.00, 3504.86),
            ("JPY", "GBP", 2500.0, 11.82),
            ("AUD", "USD", 75.25, 50.23),
            ("KRW", "SGD", 999.99, 0.88),
            ("EUR", "THB", 10.00, 366.32),
            ("MYR", "USD", 500.00, 122.74),
            ("CHF", "NOK", 33.33, 421.42),
            ("PLN", "HKD", 88.80, 191.27),
            ("ZAR", "CAD", 100.00, 8.38),
        ];
        for (from, to, amount, expected) in cases {
            let conversion_amount = get_conversion_amount(rates.clone(), amount, to, from)
                .await
                .unwrap();
            assert_eq!(conversion_amount, expected);
        }
    }
}
