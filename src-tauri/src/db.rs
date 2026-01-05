use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{path::PathBuf, str::FromStr};
use tauri::{AppHandle, Manager};

use crate::AppConfig;

pub struct Db(pub SqlitePool);

pub async fn init_db(app: &AppHandle) -> Result<SqlitePool, String> {
    let config = app.state::<AppConfig>();
    // Put DB under the app data directory (in mobile it maps to the app sandbox)
    let app_data_dir = set_app_data_dir(app, &config).await?;

    let db_url = get_db_url(app_data_dir, &config);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(&db_url)
                .map_err(|e| format!("failed to parse db url: {e}"))?
                .create_if_missing(true)
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal),
        )
        .await
        .map_err(|e| format!("db connect error: {e}"))?;

    // Apply migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| format!("db migrate error: {e}"))?;

    Ok(pool)
}

async fn set_app_data_dir(app: &AppHandle, config: &AppConfig) -> Result<PathBuf, String> {
    // On mobile, we must use the app_data_dir regardless of env setting because we can't write to current_dir
    if cfg!(mobile) {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("app data dir error: {e}"))?;
        std::fs::create_dir_all(&app_data_dir).map_err(|e| format!("create dir error: {e}"))?;
        return Ok(app_data_dir);
    }

    // On desktop, we can use the app_data_dir based on env setting
    // if env is dev , the app_data_dir is src-tauri/app_data_dir
    // if env is prod , the app_data_dir is as per tauri default based on the platform
    let app_data_dir = match config.env.as_str() {
        "dev" => std::env::current_dir().unwrap().join("app_data_dir"),
        "prod" => app
            .path()
            .app_data_dir()
            .map_err(|e| format!("app data dir error: {e}"))?,
        _ => app
            .path()
            .app_data_dir()
            .map_err(|e| format!("app data dir error: {e}"))?,
    };
    std::fs::create_dir_all(&app_data_dir).map_err(|e| format!("create dir error: {e}"))?;
    Ok(app_data_dir)
}

fn get_db_url(app_data_dir: PathBuf, config: &AppConfig) -> String {
    let name = match config.env.as_str() {
        "dev" => "aura_dev".to_owned(),
        "prod" => "aura".to_owned(),
        _ => "aura".to_owned(),
    };
    let path = app_data_dir.join(format!("{}.sqlite", name));
    format!("sqlite:{}", path.to_string_lossy())
}
