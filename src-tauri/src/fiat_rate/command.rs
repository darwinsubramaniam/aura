use crate::db::Db;
use crate::fiat_exchanger::frankfurter_exchanger::FrankfurterExchangerApi;
use crate::fiat_rate::{get_rate, FiatExchangeRate};
use anyhow::Result;
use chrono::NaiveDate;

#[tauri::command]
async fn get_fiat_rate(db: tauri::State<'_, Db>, date: &NaiveDate) -> Result<FiatExchangeRate> {
    let exchange_api = FrankfurterExchangerApi::default();

    let rate = get_rate(&db, &exchange_api, date, None).await?;

    Ok(rate)
}
