use sqlx::{Error, SqlitePool};

pub async fn init_pool() -> Result<sqlx::Pool<sqlx::Sqlite>, Error> {
    SqlitePool::connect("sqlite://inventory.db?mode=rwc").await
}
