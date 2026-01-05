use crate::db::Db;
use crate::fiat_ramp::CreateFiatRamp;
use crate::fiat_ramp::FiatRampService;
use tauri::State;

#[tauri::command]
pub async fn create_fiat_ramp(
    create_fiat_ramp: CreateFiatRamp,
    db: State<'_, Db>,
) -> Result<(), String> {
    println!("create_fiat_ramp: {:?}", create_fiat_ramp);
    FiatRampService::create_fiat_ramp(create_fiat_ramp, &db)
        .await
        .map_err(|e| format!("failed to create fiat ramp: {e}"))
}
