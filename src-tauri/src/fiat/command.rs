use tauri::State;

use crate::{
    db::Db,
    fiat::{Fiat, FiatService},
};

#[tauri::command]
pub async fn get_all_fiat(db: State<'_, Db>) -> Result<Vec<Fiat>, String> {
    let fiat = FiatService::get_all_fiat(&db)
        .await
        .map_err(|e| format!("failed to get all fiat: {e}"))?;
    Ok(fiat)
}

#[tauri::command]
pub async fn get_available_currencies(
    db: State<'_, Db>,
    symbol: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<Fiat>, String> {
    // Symbol if present should be in Capital letters
    let symbol = symbol.map(|s| s.to_uppercase());
    let fiat = FiatService::get_fiat(&db, symbol, limit, offset)
        .await
        .map_err(|e| format!("failed to get available currencies: {e}"))?;
    Ok(fiat)
}

#[tauri::command]
pub async fn get_total_count(db: State<'_, Db>) -> Result<i64, String> {
    let total_count = FiatService::get_total_count(&db)
        .await
        .map_err(|e| format!("failed to get total count: {e}"))?;
    Ok(total_count)
}
