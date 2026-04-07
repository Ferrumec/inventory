mod config;
mod handlers;
mod models;
mod services;
mod subscriptions;
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
            Ok(_r) => true,
            Err(_e) => false,
        }
    }
}
