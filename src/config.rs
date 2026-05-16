use std::sync::Arc;

use crate::handlers;
use crate::services::InventoryService;
use crate::subscriptions::subscribe;
use actix_web::web;
use actixutils::OrphanWrapper;
use actixutils::{Identity, Validate};
use event_stream::EventStream;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Sqlite;

#[derive(Clone)]
pub struct InventoryModule {
    pub service: Arc<InventoryService>,
    validator: Arc<dyn Validate<Identity>>,
}

impl InventoryModule {
    pub async fn new(
        es: event_stream::OrphanWrapper<Arc<dyn EventStream>>,
        pool: Pool<Sqlite>,
        validator: OrphanWrapper<Arc<dyn Validate<Identity>>>,
    ) -> Result<Self, Error> {
        let service = Arc::new(InventoryService::new(pool.clone(), es.0.clone())?);
        subscribe(es.0, service.clone()).await;
        Ok(Self {
            service,
            validator: validator.0,
        })
    }

    pub fn config(&self, cfg: &mut web::ServiceConfig, namespace: &str) {
        cfg.service(
            web::scope(namespace)
                .app_data(self.service.clone())
                .route("/items", web::get().to(handlers::list_items))
                .route("/items", web::post().to(handlers::create_item))
                .route(
                    "/items/{id}/reserve/{qty}",
                    web::post().to(handlers::reserve_item),
                )
                .route(
                    "/items/{id}/release/{qty}",
                    web::post().to(handlers::release_item),
                )
                .route(
                    "/items/{id}/commit/{qty}",
                    web::post().to(handlers::commit_item),
                ),
        );
    }
}
