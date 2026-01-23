use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedCryptoCoin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ExchangeRateRequest {
    pub crypto_coin_id: i64,
    pub fiat_id: i64,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRateResponse {
    pub rate: Decimal,
    pub date: NaiveDate,
}
