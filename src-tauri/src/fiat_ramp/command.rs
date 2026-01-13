use crate::db::Db;
use crate::db::StringRowId;
use crate::fiat_exchanger::frankfurter_exchanger::FrankfurterExchangerApi;
use crate::fiat_ramp::CreateFiatRamp;
use crate::fiat_ramp::FiatRampPagination;
use crate::fiat_ramp::FiatRampService;
use crate::fiat_ramp::UpdateFiatRamp;
use crate::fiat_rate;

use tauri::State;

#[tauri::command]
pub async fn create_fiat_ramp(
    create_fiat_ramp: CreateFiatRamp,
    db: State<'_, Db>,
) -> Result<StringRowId, String> {
    let date = create_fiat_ramp.ramp_date;
    let fiat_id = create_fiat_ramp.fiat_id;

    let result = FiatRampService::create(create_fiat_ramp, &db)
        .await
        .map_err(|e| format!("failed to create fiat ramp: {e}"))?;

    // Trigger rate fetch
    let api = FrankfurterExchangerApi::default();
    let _ = fiat_rate::get_rate(&db, &api, fiat_id, &date, Some(&result)).await.ok();

    Ok(result)
}

/// Get all fiat ramps with pagination -- limit and offset are optional but default to 50 and 0 respectively
#[tauri::command]
pub async fn get_fiat_ramps(
    db: State<'_, Db>,
    limit: Option<u32>,
    offset: Option<u32>,
    query: Option<String>,
) -> Result<FiatRampPagination, String> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    FiatRampService::get(limit, offset, query, &db)
        .await
        .map_err(|e| format!("failed to get all fiat ramps: {e}"))
}

/// Update a fiat ramp
#[tauri::command]
pub async fn update_fiat_ramp(fiat_ramp: UpdateFiatRamp, db: State<'_, Db>) -> Result<u64, String> {
    let id = fiat_ramp.id.clone();
    
    // We need to fetch the updated values (or existing values if partial update) to trigger rate check.
    // However, UpdateFiatRamp has Option fields.
    // For simplicity, we just perform the update first.
    
    let rows_affected = FiatRampService::update(fiat_ramp, &db)
        .await
        .map_err(|e| format!("failed to update fiat ramp: {e}"))?;

    // Now fetch the updated ramp to get the full state (fiat_id, ramp_date)
    // We can't easily fetch just one ramp with existing service (get returns pagination).
    // Let's implement a quick fetch or just use raw query here? Or add get_by_id to service.
    // Given instructions, I'll use raw query to be safe and quick.
    
    // Actually, we can just trigger get_rate IF fiat_id or ramp_date were updated.
    // But we need BOTH values to call get_rate.
    // So we must fetch the row.
    
    let ramp_row = sqlx::query("SELECT fiat_id, ramp_date FROM fiat_ramp WHERE id = ?")
        .bind(&id)
        .fetch_optional(&db.0)
        .await
        .map_err(|e| format!("failed to fetch updated ramp: {e}"))?;

    if let Some(row) = ramp_row {
        use sqlx::Row;
        let fiat_id: i64 = row.get("fiat_id");
        let ramp_date: chrono::NaiveDate = row.get("ramp_date");
        let api = FrankfurterExchangerApi::default();
        // Trigger get_rate. This handles queue updates (removal/upsert) internally.
        let _ = fiat_rate::get_rate(&db, &api, fiat_id, &ramp_date, Some(&id)).await.ok();
    }

    Ok(rows_affected)
}

/// Delete a fiat ramp
#[tauri::command]
pub async fn delete_fiat_ramp(id: StringRowId, db: State<'_, Db>) -> Result<u64, String> {
    FiatRampService::delete(id, &db)
        .await
        .map_err(|e| format!("failed to delete fiat ramp: {e}"))
}
