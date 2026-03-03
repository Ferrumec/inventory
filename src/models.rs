use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct InventoryItem {
    pub id: String,
    pub sku: String,
    pub total_quantity: i32,
    pub reserved_quantity: i32,
}

#[derive(Deserialize)]
pub struct UpdateQuantity {
    pub quantity: i32,
}
