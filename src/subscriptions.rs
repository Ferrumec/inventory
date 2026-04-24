use std::sync::Arc;

use event_stream::{EventStream, Handler};
use ferrumec::async_trait;
use serde::{Deserialize, Serialize};

use crate::services::InventoryService;

struct OnNewOrder {
    service: Arc<InventoryService>,
}

impl OnNewOrder {
    fn new(service: Arc<InventoryService>) -> Self {
        Self { service }
    }
}

#[derive(Serialize, Deserialize)]
struct OrderItem {
    id: String,
    qty: u32,
}
#[derive(Serialize, Deserialize)]
struct NewOrder {
    id: String,
    items: Vec<OrderItem>,
}

#[async_trait]
impl Handler for OnNewOrder {
    async fn handle(&self, subject:String, message: Vec<u8>) {
        let new_order: NewOrder = serde_json::from_str(str::from_utf8(&message).unwrap()).unwrap();
        for item in &new_order.items {
            let _ = self.service.reserve_item(&item.id, item.qty).await;
        }
        let _ = self
            .service
            .es
            .publish(
                "order-reserved".to_owned(),
                serde_json::to_string(&new_order).unwrap().into_bytes(),
            )
            .await;
    }
}

pub async fn subscribe(es: Arc<dyn EventStream>, service: Arc<InventoryService>) {
    let on_new_order = OnNewOrder::new(service);
    let _ = es
        .subscribe("new-order".to_owned(), Arc::new(on_new_order))
        .await;
}
