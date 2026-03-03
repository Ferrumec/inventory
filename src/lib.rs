mod config;
mod db;
mod handlers;
mod models;
mod services;

pub use config::InventoryModule;
use ferrumec::{OnCreateHandler, async_trait};

use ferrumec::CreateItem;

pub struct CreateItemOnInventory {
    pub service: services::InventoryService,
}

#[async_trait]
impl OnCreateHandler for CreateItemOnInventory {
    type Dto = CreateItem;
    async fn handle(&self, item: CreateItem) -> bool {
        match self.service.create_item(&item).await {
            Ok(r) => true,
            Err(e) => false,
        }
    }
}
