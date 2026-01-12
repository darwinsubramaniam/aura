use crate::{
    db::Db,
    fiat::{Fiat, FiatService},
    fiat_exchanger::frankfurter_exchanger::FrankfurterExchangerApi,
};
use tauri::State;

#[tauri::command]
pub async fn get_all_currencies(db: State<'_, Db>) -> Result<Vec<Fiat>, String> {
    let fiat = FiatService::<FrankfurterExchangerApi>::get_all_fiat(&db)
        .await
        .map_err(|e| format!("failed to get all fiat: {e}"))?;
    Ok(fiat)
}

#[tauri::command]
pub async fn get_currencies_by_symbol(db: State<'_, Db>, symbol: String) -> Result<Fiat, String> {
    let fiat = FiatService::<FrankfurterExchangerApi>::get_fiat_by_symbol(
        &db,
        symbol.to_uppercase().as_str(),
    )
    .await
    .map_err(|e| format!("failed to get available currencies: {e}"))?;
    Ok(fiat)
}
