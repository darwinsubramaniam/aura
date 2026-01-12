use crate::db::Db;
use crate::fiat_exchanger::frankfurter_exchanger::FrankfurterExchangerApi;
use crate::user_settings::{ensure_exists, update};
use crate::user_settings::{UpdateUserSettings, UserSettings};
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn get_user_settings(db: State<'_, Db>) -> Result<UserSettings, String> {
    let user_settings = ensure_exists::<FrankfurterExchangerApi>(&db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(user_settings)
}

#[tauri::command]
pub async fn update_user_settings(
    db: State<'_, Db>,
    user_settings: UpdateUserSettings,
) -> Result<UserSettings, String> {
    update(user_settings, &db).await.map_err(|e| e.to_string())
}
