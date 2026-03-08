use crate::models::InventoryItem;
use ferrumec::CreateItem;
use sqlx::{Error, SqlitePool};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

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
    pub async fn init_schema(pool: &SqlitePool) -> Result<(), Error> {
        MIGRATOR.run(pool).await.map_err(sqlx::Error::from)
    }
    pub async fn new(pool: SqlitePool) -> Result<Self, Error> {
        InventoryService::init_schema(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn create_item(&self, item: &CreateItem) -> Result<InventoryItem, ServiceError> {
        let total_quantity = i64::from(item.quantity);
        sqlx::query!(
            "INSERT INTO inventory (id, sku, total_quantity, reserved_quantity) VALUES (?, ?, ?, 0)",
            item.id,
            item.sku,
            total_quantity,
        )
        .execute(&self.pool)
        .await?;

        Ok(InventoryItem {
            name: item.name.clone(),
            id: item.id.clone(),
            sku: item.sku.clone(),
            total_quantity: item.quantity as i64,
            reserved_quantity: 0,
        })
    }

    pub async fn list_items(&self) -> Result<Vec<InventoryItem>, ServiceError> {
        let rows = sqlx::query_as!(
            InventoryItem,
            "SELECT id, name, sku, reserved_quantity, total_quantity FROM inventory"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn reserve_item(&self, id: &str, qty: u32) -> Result<(), ServiceError> {
        if qty <= 0 {
            return Err(ServiceError::BadRequest("Quantity must be positive".into()));
        }

        let qty_i64 = i64::from(qty);
        let result = sqlx::query!(
            "UPDATE inventory
             SET reserved_quantity = reserved_quantity + ?
             WHERE id = ?
             AND total_quantity - reserved_quantity >= ?",
            qty_i64,
            id,
            qty_i64,
        )
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

        let qty_i64 = i64::from(qty);
        let result = sqlx::query!(
            "UPDATE inventory
             SET reserved_quantity = reserved_quantity - ?
             WHERE id = ?
             AND reserved_quantity >= ?",
            qty_i64,
            id,
            qty_i64,
        )
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

        let qty_i64 = i64::from(qty);
        let result = sqlx::query!(
            "UPDATE inventory
             SET reserved_quantity = reserved_quantity - ?,
                 total_quantity = total_quantity - ?
             WHERE id = ?
             AND reserved_quantity >= ?",
            qty_i64,
            qty_i64,
            id,
            qty_i64,
        )
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
