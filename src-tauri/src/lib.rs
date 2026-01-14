mod db;
mod fiat;
mod fiat_exchanger;
mod fiat_ramp;
mod fiat_rate;
mod sys_tracker;
mod user_settings;
mod utils;

use crate::{
    db::{init_db, Db},
    fiat_exchanger::frankfurter_exchanger::FrankfurterExchangerApi,
};
use fiat::command as fiat_command;
use fiat_ramp::command as fiat_ramp_command;
use tauri::Manager;
use user_settings::command as user_settings_command;

#[derive(Default)]
pub struct AppConfig {
    env: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let env = std::env::var("ENV").unwrap_or_else(|_| "dev".into());

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            let handle = app.handle().clone();
            //1. Initialize App Config - this should be done first as it is used by other components
            handle.manage(AppConfig { env });
            //2. Initialize Database Pool Connection - this should be done second as it depends on App Config
            tauri::async_runtime::block_on(async move {
                let pool = init_db(&handle).await?;
                let db = Db(pool);
                if let Err(e) = fiat::FiatService::<FrankfurterExchangerApi>::default()
                    .update_currencies(&db)
                    .await
                {
                    eprintln!("Failed to update currencies: {}", e);
                }

                if let Err(e) = user_settings::ensure_exists::<FrankfurterExchangerApi>(&db).await {
                    eprintln!("Failed to ensure user settings: {}", e);
                }

                // Background task: Process missing rates queue
                let db_for_task = Db(db.0.clone());
                tauri::async_runtime::spawn(async move {
                    let api = FrankfurterExchangerApi::default();
                    if let Err(e) =
                        crate::fiat_rate::process_missing_rates(&db_for_task, &api).await
                    {
                        eprintln!("Failed to process missing rates: {}", e);
                    }
                });

                handle.manage(db);
                Ok::<(), String>(())
            })?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fiat_command::get_all_currencies,
            fiat_command::get_currencies_by_symbol,
            fiat_ramp_command::create_fiat_ramp,
            fiat_ramp_command::get_fiat_ramps,
            fiat_ramp_command::update_fiat_ramp,
            fiat_ramp_command::delete_fiat_ramp,
            fiat_ramp_command::get_fiat_ramp_summary,
            fiat_ramp_command::get_fiat_ramp_date_range,
            user_settings_command::get_user_settings,
            user_settings_command::update_user_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
