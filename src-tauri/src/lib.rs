mod db;
mod fiat;
mod fiat_ramp;
mod fiat_rate;
mod sys_tracker;
mod utils;

use crate::db::{init_db, Db};
use fiat::command as fiat_command;
use fiat_ramp::command as fiat_ramp_command;
use tauri::Manager;

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
                if let Err(e) = fiat::FiatService::update_currencies(&db).await {
                    eprintln!("Failed to update currencies: {}", e);
                }
                handle.manage(db);
                Ok::<(), String>(())
            })?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fiat_command::get_available_currencies,
            fiat_command::get_all_fiat,
            fiat_ramp_command::create_fiat_ramp,
            fiat_ramp_command::get_all_fiat_ramps,
            fiat_ramp_command::update_fiat_ramp,
            fiat_ramp_command::delete_fiat_ramp,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
