use crate::{
    crypto_asset::{
        error::CryptoAssetError,
        models::{CreateCryptoAsset, CryptoAsset, CryptoAssetPagination, UpdateCryptoAsset},
    },
    db::{Db, RowId},
    utils::pagination_model::{Pagination, SortDirection, SortOptions},
};

pub struct CryptoAssetService {}

// Purpose is to enable the interal application service to add CRUD with the Database layer
impl CryptoAssetService {
    pub async fn get(db: &Db, pagination: &Pagination) -> Result<CryptoAssetPagination, String> {
        let query_filter = pagination.query.as_deref().unwrap_or_default();

        let order_by = match &pagination.sort {
            Some(SortOptions {
                column: Some(col),
                direction,
            }) => {
                let valid_column = match col.as_str() {
                    "name" => "name",
                    "symbol" => "symbol",
                    "kind" => "kind",
                    "created_at" => "created_at",
                    "updated_at" => "updated_at",
                    _ => "updated_at",
                };
                let dir = match direction.clone().unwrap_or_default() {
                    SortDirection::Asc => "ASC",
                    SortDirection::Desc => "DESC",
                };
                format!("ORDER BY {} {}", valid_column, dir)
            }
            _ => "ORDER BY updated_at DESC".to_string(),
        };

        // Use a Window Function to get the total count in the same query.
        // This avoids scanning the table twice (once for count, once for data).
        let sql = format!(
            r#"
            SELECT
                id, name, symbol, kind, created_at, updated_at,
                COUNT(*) OVER() as total_count
            FROM asset
            WHERE name LIKE ? OR symbol LIKE ?
            {}
            LIMIT ? OFFSET ?
            "#,
            order_by
        );

        // Define a temporary struct to map the result including the count
        #[derive(sqlx::FromRow)]
        struct AssetWithCount {
            id: RowId,
            name: String,
            symbol: String,
            kind: crate::crypto_asset::models::CryptoAssetType,
            created_at: chrono::NaiveDateTime,
            updated_at: chrono::NaiveDateTime,
            total_count: i64,
        }

        let results = sqlx::query_as::<sqlx::Sqlite, AssetWithCount>(&sql)
            .bind(format!("%{}%", query_filter))
            .bind(format!("%{}%", query_filter))
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&db.0)
            .await
            .map_err(|e| format!("failed to get assets: {e}"))?;

        // If results are empty, count is 0. Otherwise, take count from the first row.
        let total_count = results.first().map(|r| r.total_count).unwrap_or(0);

        let assets = results
            .into_iter()
            .map(|r| CryptoAsset {
                id: r.id,
                name: r.name,
                symbol: r.symbol,
                kind: r.kind,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect();

        Ok(CryptoAssetPagination {
            total_count,
            assets,
        })
    }

    pub async fn get_by_id(db: &Db, id: RowId) -> Result<CryptoAsset, CryptoAssetError> {
        let result = sqlx::query_as::<_, CryptoAsset>(
            r#"
                SELECT id, name, symbol, kind, created_at, updated_at
                FROM asset
                WHERE id = ?
                "#,
        )
        .bind(id)
        .fetch_one(&db.0)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                CryptoAssetError::AssetNotFound(format!("Crypto Coin with ID: {id}"))
            }
            _ => CryptoAssetError::Other(format!("Failed to get asset by ID: {id}")),
        })?;

        Ok(result)
    }

    pub async fn create(db: &Db, data: &CreateCryptoAsset) -> Result<RowId, String> {
        let id = sqlx::query_scalar(
            r#"
            INSERT INTO asset (name, symbol, kind)
            VALUES (?, ?, ?)
            RETURNING id
            "#,
        )
        .bind(&data.name)
        .bind(&data.symbol)
        .bind(&data.kind)
        .fetch_one(&db.0)
        .await
        .map_err(|e| format!("failed to create asset: {e}"))?;

        Ok(id)
    }

    pub async fn update(db: &Db, id: RowId, data: &UpdateCryptoAsset) -> Result<u64, String> {
        let result = sqlx::query(
            r#"
            UPDATE asset
            SET
                name = COALESCE(?, name),
                symbol = COALESCE(?, symbol),
                kind = COALESCE(?, kind),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(&data.name)
        .bind(&data.symbol)
        .bind(&data.kind)
        .bind(id)
        .execute(&db.0)
        .await
        .map_err(|e| format!("failed to update asset: {e}"))?;

        Ok(result.rows_affected())
    }

    pub async fn delete(db: &Db, id: RowId) -> Result<u64, String> {
        let result = sqlx::query("DELETE FROM asset WHERE id = ?")
            .bind(id)
            .execute(&db.0)
            .await
            .map_err(|e| format!("failed to delete asset: {e}"))?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_asset::models::CryptoAssetType;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::str::FromStr;

    async fn init_db() -> Db {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:")
                    .unwrap()
                    .create_if_missing(true)
                    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal),
            )
            .await
            .unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        Db(pool)
    }

    #[tokio::test]
    async fn test_create_asset() {
        let db = init_db().await;
        let create_asset = CreateCryptoAsset {
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            kind: CryptoAssetType::Crypto,
        };
        let result = CryptoAssetService::create(&db, &create_asset).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_assets() {
        let db = init_db().await;
        let create_asset = CreateCryptoAsset {
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            kind: CryptoAssetType::Crypto,
        };
        CryptoAssetService::create(&db, &create_asset)
            .await
            .unwrap();

        let pagination = Pagination {
            limit: 10,
            offset: 0,
            query: None,
            sort: None,
        };

        let result = CryptoAssetService::get(&db, &pagination).await.unwrap();
        assert_eq!(result.total_count, 1);
        assert_eq!(result.assets.len(), 1);
        assert_eq!(result.assets[0].symbol, "BTC");
    }

    #[tokio::test]
    async fn test_update_asset() {
        let db = init_db().await;
        let create_asset = CreateCryptoAsset {
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            kind: CryptoAssetType::Crypto,
        };
        let id = CryptoAssetService::create(&db, &create_asset)
            .await
            .unwrap();

        let update_asset = UpdateCryptoAsset {
            name: Some("Bitcoin Updated".to_string()),
            symbol: None,
            kind: None,
        };

        let result = CryptoAssetService::update(&db, id, &update_asset).await;
        assert!(result.is_ok());

        let pagination = Pagination {
            limit: 10,
            offset: 0,
            query: None,
            sort: None,
        };
        let result = CryptoAssetService::get(&db, &pagination).await.unwrap();
        assert_eq!(result.assets[0].name, "Bitcoin Updated");
    }

    #[tokio::test]
    async fn test_delete_asset() {
        let db = init_db().await;
        let create_asset = CreateCryptoAsset {
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            kind: CryptoAssetType::Crypto,
        };
        let id = CryptoAssetService::create(&db, &create_asset)
            .await
            .unwrap();

        let result = CryptoAssetService::delete(&db, id).await;
        assert!(result.is_ok());

        let pagination = Pagination {
            limit: 10,
            offset: 0,
            query: None,
            sort: None,
        };
        let result = CryptoAssetService::get(&db, &pagination).await.unwrap();
        assert_eq!(result.total_count, 0);
    }
}
