pub mod command;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    db::{Db, RowId},
    fiat::FiatService,
};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserSettings {
    pub id: RowId,
    pub locale: String,
    pub default_fiat_id: RowId,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct CreateUserSettings {
    pub locale: String,
    pub default_fiat_id: RowId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserSettings {
    pub locale: Option<String>,
    pub default_fiat_id: Option<RowId>,
}

const DEFAULT_LOCALE: &str = "en";
const DEFAULT_DEFAULT_FIAT_SYMBOL: &str = "USD";
const ONLY_ONE_USER_SETTINGS: RowId = 1;

/// Ensure user settings exist, creating them if necessary.
pub async fn ensure_exists(db: &Db) -> Result<UserSettings> {
    if exists(db).await? {
        return get(db).await;
    }

    let default_fiat_id = FiatService::get_fiat_by_symbol(db, DEFAULT_DEFAULT_FIAT_SYMBOL)
        .await
        .context("failed to get default fiat")?
        .id;

    let create_user_settings = CreateUserSettings {
        locale: DEFAULT_LOCALE.to_owned(),
        default_fiat_id,
    };

    create(create_user_settings, db).await
}

/// Create a new user settings record
pub async fn create(data: CreateUserSettings, db: &Db) -> Result<UserSettings> {
    sqlx::query_as::<sqlx::Sqlite, UserSettings>(
        "INSERT INTO user_settings (id, locale, default_fiat_id) VALUES (?, ?, ?) RETURNING *",
    )
    .bind(ONLY_ONE_USER_SETTINGS)
    .bind(data.locale)
    .bind(data.default_fiat_id)
    .fetch_one(&db.0)
    .await
    .context("failed to insert into user_settings table")
}

/// Check if user settings exist
pub async fn exists(db: &Db) -> Result<bool> {
    let result = sqlx::query("SELECT 1 FROM user_settings LIMIT 1")
        .fetch_optional(&db.0)
        .await
        .context("failed to check user_settings existence")?;
    Ok(result.is_some())
}

/// Update user settings
pub async fn update(data: UpdateUserSettings, db: &Db) -> Result<UserSettings> {
    sqlx::query_as::<sqlx::Sqlite, UserSettings>(
        "UPDATE user_settings SET locale = COALESCE(?, locale), default_fiat_id = COALESCE(?, default_fiat_id) WHERE id = ? RETURNING *",
    )
    .bind(data.locale)
    .bind(data.default_fiat_id)
    .bind(ONLY_ONE_USER_SETTINGS)
    .fetch_one(&db.0)
    .await
    .context("failed to update user_settings table")
}

pub async fn get(db: &Db) -> Result<UserSettings> {
    sqlx::query_as::<sqlx::Sqlite, UserSettings>("SELECT * FROM user_settings")
        .fetch_one(&db.0)
        .await
        .context("failed to get user_settings")
}
