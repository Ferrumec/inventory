use actix_web::{HttpResponse, Responder, web};
use ferrumec::CreateItem;

use crate::services::InventoryService;
use crate::services::ServiceError;

fn service_error_to_response(err: ServiceError) -> HttpResponse {
    match err {
        ServiceError::NotFound => HttpResponse::NotFound().body("Item not found"),
        ServiceError::BadRequest(message) => HttpResponse::BadRequest().body(message),
        ServiceError::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create_item(
    pool: web::Data<InventoryService>,
    item: web::Json<CreateItem>,
) -> impl Responder {
    match pool.create_item(&item).await {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(err) => service_error_to_response(err),
    }
}

pub async fn list_items(pool: web::Data<InventoryService>) -> impl Responder {
    match pool.list_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => service_error_to_response(err),
    }
}

pub async fn reserve_item(
    pool: web::Data<InventoryService>,
    path: web::Path<(String, u32)>,
) -> impl Responder {
    let (id, qty) = path.into_inner();
    match pool.reserve_item(&id, qty).await {
        Ok(()) => HttpResponse::Ok().body("Reserved successfully"),
        Err(err) => service_error_to_response(err),
    }
}

pub async fn release_item(
    pool: web::Data<InventoryService>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let (id, qty) = path.into_inner();
    match pool.release_item(&id, qty).await {
        Ok(()) => HttpResponse::Ok().body("Released successfully"),
        Err(err) => service_error_to_response(err),
    }
}

pub async fn commit_item(
    pool: web::Data<InventoryService>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let (id, qty) = path.into_inner();
    match pool.commit_item(&id, qty).await {
        Ok(()) => HttpResponse::Ok().body("Committed successfully"),
        Err(err) => service_error_to_response(err),
    }
}
