mod db;
mod fiat;
mod sys_tracker;
mod utils;

use crate::db::{init_db, Db};
use fiat::command;
use tauri::Manager;

#[derive(Default)]
pub struct AppConfig {
    env: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let env = std::env::var("ENV").unwrap_or_else(|_| "dev".into());

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            //1. Initialize App Config - this should be done first as it is used by other components
            handle.manage(AppConfig { env });
            //2. Initialize Database Pool Connection - this should be done second as it depends on App Config
            tauri::async_runtime::block_on(async move {
                let pool = init_db(&handle).await?;
                let db = Db(pool);
                fiat::FiatService::update_currencies(&db).await?;
                handle.manage(db);
                Ok::<(), String>(())
            })?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::get_available_currencies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
