use crate::db::Db;
use sqlx::Row;

pub struct SysTracker {}

impl SysTracker {
    pub async fn get_last_updated_at(
        name: &str,
        db: &Db,
    ) -> Result<Option<chrono::NaiveDateTime>, String> {
        // check if table exists in the sys_tracker table
        let last_updated_at = sqlx::query("SELECT last_updated_at FROM sys_tracker WHERE name=?")
            .bind(name)
            .fetch_one(&db.0)
            .await;
        if last_updated_at.is_err() {
            return Ok(None);
        }
        let last_updated_at = last_updated_at
            .unwrap()
            .get::<chrono::NaiveDateTime, _>("last_updated_at");
        Ok(Some(last_updated_at))
    }

    pub async fn update_last_updated_at(name: &str, db: &Db) -> Result<(), String> {
        sqlx::query("INSERT OR REPLACE INTO sys_tracker (name, last_updated_at) VALUES (?, ?)")
            .bind(name)
            .bind(chrono::Local::now().naive_local())
            .execute(&db.0)
            .await
            .map_err(|e| format!("failed to update last updated at: {e}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use sqlx::sqlite::SqliteConnectOptions;

    use super::*;
    use crate::db::Db;

    async fn setup() -> Db {
        // create a test db
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(SqliteConnectOptions::new().in_memory(true))
            .await
            .expect("Failed to create in-memory database");

        let db = Db(pool);
        // create sys_tracker table
        sqlx::query("CREATE TABLE IF NOT EXISTS sys_tracker (id TEXT PRIMARY KEY, table_name TEXT NOT NULL, last_updated_at TIMESTAMP NOT NULL)")
            .execute(&db.0)
            .await
            .map_err(|e| format!("failed to create sys_tracker table: {e}"))
            .unwrap();

        // create some data into sys_tracker table
        let less_than_24_hours_ago: chrono::NaiveDateTime =
            chrono::Local::now().naive_local() - chrono::Duration::hours(23);
        sqlx::query("INSERT INTO sys_tracker (id, table_name, last_updated_at) VALUES (?, ?, ?)")
            .bind("1")
            .bind("table_updated_less_than_24_hours_ago")
            .bind(less_than_24_hours_ago)
            .execute(&db.0)
            .await
            .map_err(|e| format!("failed to insert into sys_tracker table: {e}"))
            .unwrap();

        let more_than_24_hours_ago: chrono::NaiveDateTime =
            chrono::Local::now().naive_local() - chrono::Duration::hours(25);
        sqlx::query("INSERT INTO sys_tracker (id, table_name, last_updated_at) VALUES (?, ?, ?)")
            .bind("2")
            .bind("table_updated_more_than_24_hours_ago")
            .bind(more_than_24_hours_ago)
            .execute(&db.0)
            .await
            .map_err(|e| format!("failed to insert into sys_tracker table: {e}"))
            .unwrap();

        db
    }

    #[tokio::test]
    async fn test_get_last_updated_at() {
        let db = setup().await;

        let last_updated_at =
            SysTracker::get_last_updated_at("table_updated_less_than_24_hours_ago", &db)
                .await
                .unwrap()
                .unwrap();

        println!("last_updated_at: {last_updated_at}");

        // check is last_updated_at is still within 24 hours
        let now = chrono::Local::now().naive_local();
        assert!(last_updated_at > now - chrono::Duration::hours(24));
    }

    #[tokio::test]
    async fn test_get_last_updated_at_more_than_24_hours_ago() {
        let db = setup().await;

        let last_updated_at =
            SysTracker::get_last_updated_at("table_updated_more_than_24_hours_ago", &db)
                .await
                .unwrap()
                .unwrap();

        println!("last_updated_at: {last_updated_at}");

        // check if last_updated_at is more than 24 hours ago
        let now = chrono::Local::now().naive_local();
        assert!(last_updated_at < now - chrono::Duration::hours(25));
    }

    #[tokio::test]
    async fn test_no_updated_status() {
        let db = setup().await;
        let last_updated_at = SysTracker::get_last_updated_at("table_no_updated_status", &db)
            .await
            .unwrap();
        assert!(last_updated_at.is_none());
    }
}
