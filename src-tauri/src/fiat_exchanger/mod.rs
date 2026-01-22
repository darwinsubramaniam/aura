pub mod frankfurter_exchanger;
use std::collections::HashMap;

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[cfg_attr(test, mockall::automock)]
pub trait FiatExchanger {
    async fn get_available_currencies(&self) -> Result<Vec<Currency>>;
    async fn get_latest_rates(&self, base: &str, date: Option<&NaiveDate>) -> Result<Rates>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub name: String,
    pub symbol: String,
}

/// Fiat currency symbol
pub type FiatSymbol = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rates {
    pub base: String,
    // the date which the rate was request / available in the server
    pub date: chrono::NaiveDate,
    pub rates: HashMap<FiatSymbol, f64>,
}
