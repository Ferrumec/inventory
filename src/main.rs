use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use auth_middleware::Auth;
use sqlx::{Row, SqlitePool};

mod models;
use models::{CreateItem, InventoryItem};

async fn create_item(pool: web::Data<SqlitePool>, item: web::Json<CreateItem>) -> impl Responder {
    let query =
        "INSERT INTO inventory (id, sku, total_quantity, reserved_quantity) VALUES (?, ?, ?, 0)";
    match sqlx::query(query)
        .bind(&item.id)
        .bind(&item.sku)
        .bind(item.quantity)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(InventoryItem {
            id: item.id.clone(),
            sku: item.sku.clone(),
            total_quantity: item.quantity,
            reserved_quantity: 0,
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn list_items(pool: web::Data<SqlitePool>) -> impl Responder {
    let query = "SELECT id, sku, total_quantity, reserved_quantity FROM inventory";
    let res = sqlx::query_as::<_, InventoryItem>(query)
        .fetch_all(pool.get_ref())
        .await;
    match res {
        Ok(rows) => {
            let items: Vec<InventoryItem> = rows;
            HttpResponse::Ok().json(items)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn reserve_item(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let (id, qty) = path.into_inner();

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let row =
        match sqlx::query("SELECT total_quantity, reserved_quantity FROM inventory WHERE id = ?")
            .bind(&id)
            .fetch_one(&mut *tx)
            .await
        {
            Ok(r) => r,
            Err(_) => return HttpResponse::NotFound().body("Item not found"),
        };

    let total: i32 = row.get("total_quantity");
    let reserved: i32 = row.get("reserved_quantity");

    if total - reserved < qty {
        return HttpResponse::BadRequest().body("Not enough stock");
    }

    if let Err(e) =
        sqlx::query("UPDATE inventory SET reserved_quantity = reserved_quantity + ? WHERE id = ?")
            .bind(qty)
            .bind(&id)
            .execute(&mut *tx)
            .await
    {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    tx.commit().await.unwrap();
    HttpResponse::Ok().body("Reserved successfully")
}

async fn release_item(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let (id, qty) = path.into_inner();
    let query = "UPDATE inventory SET reserved_quantity = reserved_quantity - ? WHERE id = ?";
    match sqlx::query(query)
        .bind(qty)
        .bind(&id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Released successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn commit_item(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let (id, qty) = path.into_inner();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let query = "SELECT total_quantity, reserved_quantity FROM inventory WHERE id = ?";
    let row = match sqlx::query(query).bind(&id).fetch_one(&mut *tx).await {
        Ok(r) => r,
        Err(_) => return HttpResponse::NotFound().body("Item not found"),
    };

    let reserved: i32 = row.get("reserved_quantity");
    if reserved < qty {
        return HttpResponse::BadRequest().body("Not enough reserved stock to commit");
    }

    let update_query = "UPDATE inventory SET reserved_quantity = reserved_quantity - ?, total_quantity = total_quantity - ? WHERE id = ?";
    match sqlx::query(update_query)
        .bind(qty)
        .bind(qty)
        .bind(&id)
        .execute(&mut *tx)
        .await
    {
        Ok(_) => {
            tx.commit().await.unwrap();
            HttpResponse::Ok().body("Committed successfully")
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("sqlite://inventory.db?mode=rwc")
        .await
        .unwrap();

    // Create table if not exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS inventory (
            id TEXT PRIMARY KEY,
            sku TEXT NOT NULL,
            total_quantity INTEGER NOT NULL,
            reserved_quantity INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("")
                .wrap(Auth)
                .route("/items", web::get().to(list_items))
                .route("/items", web::post().to(create_item))
                .route("/items/{id}/reserve/{qty}", web::post().to(reserve_item))
                .route("/items/{id}/release/{qty}", web::post().to(release_item))
                .route("/items/{id}/commit/{qty}", web::post().to(commit_item)),
        )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
