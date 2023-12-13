use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx::PgPool;

use crate::ShuttleResult;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(sql)
        .service(reset)
        .service(orders)
        .service(orders_total)
        .service(orders_popular);
}

#[get("/13/sql")]
async fn sql(pool: web::Data<PgPool>) -> ShuttleResult<impl Responder> {
    Ok(sqlx::query!("SELECT 20231213 number")
        .fetch_one(pool.as_ref())
        .await?
        .number
        .unwrap()
        .to_string())
}

#[post("/13/reset")]
async fn reset(pool: web::Data<PgPool>) -> ShuttleResult<impl Responder> {
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(pool.as_ref())
        .await?;

    sqlx::query!(
        "CREATE TABLE orders (
  id INT PRIMARY KEY,
  region_id INT,
  gift_name VARCHAR(50),
  quantity INT
)"
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[post("/13/orders")]
async fn orders(
    orders: web::Json<Vec<Order>>,
    pool: web::Data<PgPool>,
) -> ShuttleResult<impl Responder> {
    for order in orders.iter() {
        sqlx::query!(
            "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
            order.id,
            order.region_id,
            order.gift_name,
            order.quantity
        )
        .execute(pool.as_ref())
        .await?;
    }

    Ok(HttpResponse::Ok())
}

#[get("/13/orders/total")]
async fn orders_total(pool: web::Data<PgPool>) -> ShuttleResult<impl Responder> {
    let json = sqlx::query!("SELECT SUM(quantity) total FROM orders")
        .fetch_one(pool.as_ref())
        .await?
        .total
        .map(|total| serde_json::json!({ "total": total }));

    Ok(HttpResponse::Ok().json(json))
}

#[get("/13/orders/popular")]
async fn orders_popular(pool: web::Data<PgPool>) -> ShuttleResult<impl Responder> {
    let name = sqlx::query!(
        "SELECT gift_name, SUM(quantity) total FROM orders GROUP BY gift_name ORDER BY total DESC LIMIT 1"
    )
    .fetch_one(pool.as_ref())
    .await.ok().and_then(|record| record.gift_name);

    Ok(HttpResponse::Ok().json(serde_json::json!({ "popular": name })))
}
