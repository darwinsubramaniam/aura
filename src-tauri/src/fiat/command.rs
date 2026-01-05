use tauri::State;

use crate::{
    db::Db,
    fiat::{Fiat, FiatService},
};

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
