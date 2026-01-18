pub mod command;
use crate::db::RowId;
use crate::{fiat_exchanger::FiatExchanger, sys_tracker::SysTracker, utils::date_utils, Db};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub struct FiatService<A: FiatExchanger> {
    fiat_api_client: A,
}

#[derive(Debug, sqlx::FromRow, Default, Serialize, Deserialize)]
pub struct Fiat {
    pub id: RowId,
    pub symbol: String,
    pub name: String,
}


const FIAT_SYS_TRACKER_NAME: &str = "fiat";

impl<A: FiatExchanger + Default> Default for FiatService<A> {
    fn default() -> Self {
        Self {
            fiat_api_client: A::default(),
        }
    }
}

impl<A: FiatExchanger> FiatService<A> {
    pub fn new(fiat_api_client: A) -> Self {
        Self { fiat_api_client }
    }
}


impl<A: FiatExchanger> FiatService<A> {
    // update database with supported currencies symbols and names
    pub async fn update_currencies(&self, db: &Db) -> Result<u8> {
        // last updated at
        let last_updated_at = SysTracker::get_last_updated_at(FIAT_SYS_TRACKER_NAME, &db)
            .await
            .context("failed to get last updated at")?;

        let require_update = date_utils::require_update(last_updated_at, chrono::Duration::hours(24));

        if !require_update {
            return Ok(0);
        }

        let current_update_at = chrono::Local::now().naive_local();

        // 1. Get available currencies from Frankfurter API
        let currencies = self
            .fiat_api_client
            .get_available_currencies()
            .await
            .context("failed to get available currencies")?;

        let mut tx = db.0.begin().await.context("failed to begin transaction")?;

        // this is safe to be in u8 which is can only have max range of 0-255
        // there is only 188 possible work currencies in current point of time in the world
        // Frankfurter API only support 30 currencies per as of 2026-Jan-12
        // 2. Update database with supported currencies symbols and names
        let mut query_builder =
            sqlx::QueryBuilder::new("INSERT OR REPLACE INTO fiat (symbol, name, updated_at) ");

        query_builder.push_values(currencies, |mut b, currency| {
            b.push_bind(currency.symbol)
                .push_bind(currency.name)
                .push_bind(current_update_at);
        });

        let result = query_builder
            .build()
            .execute(&mut *tx)
            .await
            .context("failed to insert into fiat table")?;

        let total_row_affected = result.rows_affected() as u8;

        tx.commit().await.context("failed to commit transaction")?;

        // 3. Update SysTracker
        SysTracker::update_last_updated_at(FIAT_SYS_TRACKER_NAME, &db)
            .await
            .context("SysTracker update error")?;
        Ok(total_row_affected)
    }

    /// get all fiat from the database
    pub async fn get_all_fiat(db: &Db) -> Result<Vec<Fiat>> {
        let fiat = sqlx::query_as::<_, Fiat>("SELECT * FROM fiat")
            .fetch_all(&db.0)
            .await
            .context("failed to get all fiat")?;
        Ok(fiat)
    }

    /// get the fiat by id from the database
    pub async fn get_fiat_by_id(db: &Db, id: i64) -> Result<Fiat> {
        let fiat = sqlx::query_as::<_, Fiat>("SELECT * FROM fiat WHERE id = ?")
            .bind(id)
            .fetch_one(&db.0)
            .await
            .context("failed to get fiat by id")?;
        Ok(fiat)
    }

    /// get the fiat by symbol from the database
    pub async fn get_fiat_by_symbol(db: &Db, symbol: &str) -> Result<Fiat> {
        let fiat = sqlx::query_as::<_, Fiat>("SELECT * FROM fiat WHERE symbol = ?")
            .bind(symbol)
            .fetch_one(&db.0)
            .await
            .context("failed to get fiat by symbol")?;
        Ok(fiat)
    }
}

#[cfg(test)]
mod tests {

    use crate::fiat_exchanger::{Currency, MockFiatExchanger};
    use sqlx::sqlite::SqliteConnectOptions;

    use super::*;

    async fn get_db() -> Db {
        // create a test db
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(SqliteConnectOptions::new().in_memory(true))
            .await
            .context("failed to create in-memory database")
            .unwrap();

        let db = Db(pool);
        //migrations
        sqlx::migrate!("./migrations")
            .run(&db.0)
            .await
            .context("failed to run migrations")
            .unwrap();
        db
    }

    async fn update_db_with_mock_data(db: &Db) {
        let mut mock_api = MockFiatExchanger::new();
        mock_api
            .expect_get_available_currencies()
            .times(1)
            .returning(|| {
                Ok(vec![
                    Currency {
                        name: "United States Dollar".to_string(),
                        symbol: "USD".to_string(),
                    },
                    Currency {
                        name: "Euro".to_string(),
                        symbol: "EUR".to_string(),
                    },
                ])
            });

        // Act
        let fiat_service = FiatService {
            fiat_api_client: mock_api,
        };
        let _ = fiat_service.update_currencies(&db).await;
    }

    #[tokio::test]
    async fn test_update_currencies() {
        // Arrange
        let db = get_db().await;
        let mut mock_api = MockFiatExchanger::new();
        mock_api
            .expect_get_available_currencies()
            .times(1)
            .returning(|| {
                Ok(vec![
                    Currency {
                        name: "United States Dollar".to_string(),
                        symbol: "USD".to_string(),
                    },
                    Currency {
                        name: "Euro".to_string(),
                        symbol: "EUR".to_string(),
                    },
                ])
            });

        // Act
        let fiat_service = FiatService {
            fiat_api_client: mock_api,
        };
        let result = fiat_service.update_currencies(&db).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_fiat_by_id() {
        // Arrange
        let db = get_db().await;
        update_db_with_mock_data(&db).await;

        // Act
        let result = FiatService::<MockFiatExchanger>::get_fiat_by_id(&db, 1).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().symbol, "USD");
    }

    #[tokio::test]
    async fn test_get_fiat_by_symbol() {
        // Arrange
        let db = get_db().await;
        update_db_with_mock_data(&db).await;

        // Act
        let result = FiatService::<MockFiatExchanger>::get_fiat_by_symbol(&db, "USD").await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "United States Dollar");
    }
}
