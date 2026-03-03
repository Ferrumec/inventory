use crate::models::InventoryItem;
use ferrumec::CreateItem;
use sqlx::{Error, SqlitePool, sqlite::SqliteQueryResult};

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    BadRequest(String),
    Db(sqlx::Error),
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        Self::Db(err)
    }
}

#[derive(Clone)]
pub struct InventoryService {
    pool: SqlitePool,
}

impl InventoryService {
    pub async fn init_schema(pool: &SqlitePool) -> Result<SqliteQueryResult, Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS inventory (
                id TEXT PRIMARY KEY,
                sku TEXT NOT NULL,
                total_quantity INTEGER NOT NULL,
                reserved_quantity INTEGER NOT NULL
            )",
        )
        .execute(pool)
        .await
    }
    pub async fn new(pool: SqlitePool) -> Result<Self, Error> {
        InventoryService::init_schema(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn create_item(&self, item: &CreateItem) -> Result<InventoryItem, ServiceError> {
        let query = "INSERT INTO inventory (id, sku, total_quantity, reserved_quantity) VALUES (?, ?, ?, 0)";
        sqlx::query(query)
            .bind(&item.id)
            .bind(&item.sku)
            .bind(item.quantity)
            .execute(&self.pool)
            .await?;

        Ok(InventoryItem {
            id: item.id.clone(),
            sku: item.sku.clone(),
            total_quantity: item.quantity as i32,
            reserved_quantity: 0,
        })
    }

    pub async fn list_items(&self) -> Result<Vec<InventoryItem>, ServiceError> {
        let query = "SELECT id, sku, total_quantity, reserved_quantity FROM inventory";
        let rows = sqlx::query_as::<_, InventoryItem>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn reserve_item(&self, id: &str, qty: u32) -> Result<(), ServiceError> {
        if qty <= 0 {
            return Err(ServiceError::BadRequest("Quantity must be positive".into()));
        }

        let result = sqlx::query(
            "UPDATE inventory
             SET reserved_quantity = reserved_quantity + ?
             WHERE id = ?
             AND total_quantity - reserved_quantity >= ?",
        )
        .bind(qty)
        .bind(id)
        .bind(qty)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::BadRequest("Not enough stock".into()));
        }

        Ok(())
    }

    pub async fn release_item(&self, id: &str, qty: i32) -> Result<(), ServiceError> {
        if qty <= 0 {
            return Err(ServiceError::BadRequest("Quantity must be positive".into()));
        }

        let result = sqlx::query(
            "UPDATE inventory
             SET reserved_quantity = reserved_quantity - ?
             WHERE id = ?
             AND reserved_quantity >= ?",
        )
        .bind(qty)
        .bind(id)
        .bind(qty)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::BadRequest(
                "Cannot release more than reserved".into(),
            ));
        }

        Ok(())
    }

    pub async fn commit_item(&self, id: &str, qty: i32) -> Result<(), ServiceError> {
        if qty <= 0 {
            return Err(ServiceError::BadRequest("Quantity must be positive".into()));
        }

        let result = sqlx::query(
            "UPDATE inventory
             SET reserved_quantity = reserved_quantity - ?,
                 total_quantity = total_quantity - ?
             WHERE id = ?
             AND reserved_quantity >= ?",
        )
        .bind(qty)
        .bind(qty)
        .bind(id)
        .bind(qty)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::BadRequest(
                "Not enough reserved stock to commit".into(),
            ));
        }

        Ok(())
    }
}
