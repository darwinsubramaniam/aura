use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedCoin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ExchangeRateRequest {
    pub coin_id: String,
    pub fiat_currency: String,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRateResponse {
    pub rate: Decimal,
    pub date: NaiveDate,
}
