use crate::db;
use crate::handlers;
use crate::services;
use crate::services::InventoryService;
use actix_web::web;
use auth_middleware::Auth;
use sqlx::Error;

#[derive(Clone)]
pub struct InventoryModule {
    pub service: InventoryService,
}

impl InventoryModule {
    pub async fn new() -> Result<Self, Error> {
        let pool = db::init_pool().await?;
        let service = services::InventoryService::new(pool.clone()).await?;
        Ok(Self { service })
    }

    pub fn config(&self, cfg: &mut web::ServiceConfig, namespace: &str) {
        cfg.service(
            web::scope(namespace)
                .app_data(web::Data::new(self.service.clone()))
                .wrap(Auth)
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
