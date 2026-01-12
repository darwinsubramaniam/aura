pub mod command;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    db::{Db, RowId},
    fiat::{Fiat, FiatService},
    fiat_exchanger::FiatExchanger,
};

#[derive(Debug, FromRow, Serialize, Default)]
pub struct UserSettings {
    pub id: RowId,
    pub locale: String,
    pub default_fiat_id: RowId,
    #[sqlx(skip)]
    pub fiat: Fiat,
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
pub async fn ensure_exists<A: FiatExchanger + Default>(db: &Db) -> Result<UserSettings> {
    if exists(db).await? {
        println!("User settings already exists");
        return get(db).await;
    }

    println!("Creating user settings");
    let default_fiat_id = FiatService::<A>::get_fiat_by_symbol(db, DEFAULT_DEFAULT_FIAT_SYMBOL)
        .await
        .context("failed to get default fiat")?
        .id;

    let create_user_settings = CreateUserSettings {
        locale: DEFAULT_LOCALE.to_owned(),
        default_fiat_id,
    };

    create::<A>(create_user_settings, db).await
}

/// Create a new user settings record
pub async fn create<A: FiatExchanger + Default>(
    data: CreateUserSettings,
    db: &Db,
) -> Result<UserSettings> {
    let mut user_settings = sqlx::query_as::<sqlx::Sqlite, UserSettings>(
        "INSERT INTO user_settings (id, locale, default_fiat_id) VALUES (?, ?, ?) RETURNING *",
    )
    .bind(ONLY_ONE_USER_SETTINGS)
    .bind(data.locale)
    .bind(data.default_fiat_id)
    .fetch_one(&db.0)
    .await
    .context("failed to insert into user_settings table")?;

    user_settings.fiat = FiatService::<A>::get_fiat_by_id(db, user_settings.default_fiat_id)
        .await
        .context("failed to get default fiat")?;

    Ok(user_settings)
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
    use sqlx::Row;
    let row = sqlx::query(
        r#"
        SELECT 
            u.id, u.locale, u.default_fiat_id,
            f.id as f_id, f.symbol as f_symbol, f.name as f_name
        FROM user_settings u
        JOIN fiat f ON u.default_fiat_id = f.id
        LIMIT 1
        "#,
    )
    .fetch_one(&db.0)
    .await
    .context("failed to get user_settings")?;

    Ok(UserSettings {
        id: row.try_get("id")?,
        locale: row.try_get("locale")?,
        default_fiat_id: row.try_get("default_fiat_id")?,
        fiat: Fiat {
            id: row.try_get("f_id")?,
            symbol: row.try_get("f_symbol")?,
            name: row.try_get("f_name")?,
        },
    })
}

#[cfg(test)]
mod test {
    use crate::fiat_exchanger::frankfurter_exchanger::FrankfurterExchangerApi;

    use super::*;

    #[tokio::test]
    async fn test_ensure_exists() {
        let db = Db::in_memory().await.unwrap();
        // loading fiat table with data
        sqlx::query(
            "INSERT INTO fiat (id, symbol, name, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(1)
        .bind("USD")
        .bind("United States Dollar")
        .bind(chrono::Local::now().naive_local())
        .bind(chrono::Local::now().naive_local())
        .execute(&db.0)
        .await
        .unwrap();

        let user_settings = ensure_exists::<FrankfurterExchangerApi>(&db).await;
        assert!(user_settings.is_ok());
    }

    #[tokio::test]
    async fn test_create() {
        let db = Db::in_memory().await.unwrap();
        // loading fiat table with data
        sqlx::query(
            "INSERT INTO fiat (id, symbol, name, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(1)
        .bind("USD")
        .bind("United States Dollar")
        .bind(chrono::Local::now().naive_local())
        .bind(chrono::Local::now().naive_local())
        .execute(&db.0)
        .await
        .unwrap();

        let user_settings = create::<FrankfurterExchangerApi>(
            CreateUserSettings {
                locale: DEFAULT_LOCALE.to_owned(),
                default_fiat_id: 1,
            },
            &db,
        )
        .await;
        assert!(user_settings.is_ok());
    }

    #[tokio::test]
    async fn test_get() {
        let db = Db::in_memory().await.unwrap();
        // Insert fiat
        sqlx::query(
            "INSERT INTO fiat (id, symbol, name, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(1)
        .bind("USD")
        .bind("United States Dollar")
        .bind(chrono::Local::now().naive_local())
        .bind(chrono::Local::now().naive_local())
        .execute(&db.0)
        .await
        .unwrap();

        // Insert user settings
        sqlx::query("INSERT INTO user_settings (id, locale, default_fiat_id) VALUES (?, ?, ?)")
            .bind(1)
            .bind("en")
            .bind(1)
            .execute(&db.0)
            .await
            .unwrap();

        let settings = get(&db).await;
        assert!(settings.is_ok());
        let s = settings.unwrap();
        assert_eq!(s.fiat.symbol, "USD");
    }
}
