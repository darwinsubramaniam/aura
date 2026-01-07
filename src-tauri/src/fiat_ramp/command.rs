use crate::db::Db;
use crate::db::StringRowId;
use crate::fiat_ramp::CreateFiatRamp;
use crate::fiat_ramp::FiatRamp;
use crate::fiat_ramp::FiatRampPagination;
use crate::fiat_ramp::FiatRampService;

use tauri::State;

#[tauri::command]
pub async fn create_fiat_ramp(
    create_fiat_ramp: CreateFiatRamp,
    db: State<'_, Db>,
) -> Result<String, String> {
    FiatRampService::create_fiat_ramp(create_fiat_ramp, &db)
        .await
        .map_err(|e| format!("failed to create fiat ramp: {e}"))
}

/// Get all fiat ramps with pagination -- limit and offset are optional but default to 50 and 0 respectively
#[tauri::command]
pub async fn get_all_fiat_ramps(
    db: State<'_, Db>,
    limit: Option<u32>,
    offset: Option<u32>,
    query: Option<String>,
) -> Result<FiatRampPagination, String> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    FiatRampService::get_fiat_ramp(limit, offset, query, &db)
        .await
        .map_err(|e| format!("failed to get all fiat ramps: {e}"))
}

/// Update a fiat ramp
#[tauri::command]
pub async fn update_fiat_ramp(fiat_ramp: FiatRamp, db: State<'_, Db>) -> Result<u64, String> {
    FiatRampService::update_fiat_ramp(fiat_ramp, &db)
        .await
        .map_err(|e| format!("failed to update fiat ramp: {e}"))
}

/// Delete a fiat ramp
#[tauri::command]
pub async fn delete_fiat_ramp(id: StringRowId, db: State<'_, Db>) -> Result<u64, String> {
    FiatRampService::delete_fiat_ramp(id, &db)
        .await
        .map_err(|e| format!("failed to delete fiat ramp: {e}"))
}
