use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct InventoryItem {
    pub id: String,
    pub name: String,
    pub sku: String,
    pub total_quantity: i64,
    pub reserved_quantity: i64,
}
