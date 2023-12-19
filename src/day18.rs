use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx::PgPool;

use crate::ShuttleResult;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(reset)
        .service(regions)
        .service(regions_total)
        .service(regions_top_list);
}

#[post("/18/reset")]
async fn reset(pool: web::Data<PgPool>) -> ShuttleResult<impl Responder> {
    sqlx::query!("DROP TABLE IF EXISTS regions")
        .execute(pool.as_ref())
        .await?;
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(pool.as_ref())
        .await?;

    sqlx::query!(
        "CREATE TABLE regions (
            id INT PRIMARY KEY,
            name VARCHAR(50)
        )"
    )
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

#[derive(serde::Deserialize)]
struct Region {
    id: i32,
    name: String,
}

#[post("/18/regions")]
async fn regions(
    regions: web::Json<Vec<Region>>,
    pool: web::Data<PgPool>,
) -> ShuttleResult<impl Responder> {
    for region in regions.iter() {
        sqlx::query!(
            "INSERT INTO regions (id, name) VALUES ($1, $2)",
            region.id,
            region.name,
        )
        .execute(pool.as_ref())
        .await?;
    }

    Ok(HttpResponse::Ok())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct RegionTotal {
    region: String,
    total: i64,
}

#[get("/18/regions/total")]
async fn regions_total(pool: web::Data<PgPool>) -> ShuttleResult<impl Responder> {
    let regions_total = sqlx::query_as!(
        RegionTotal,
        "SELECT 
            name \"region!\", SUM(quantity) \"total!\" 
        FROM 
            orders 
        JOIN 
            regions 
        ON 
            region_id = regions.id 
        GROUP BY 
            name 
        ORDER BY 
            name"
    )
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(regions_total))
}

#[derive(serde::Serialize)]
struct RegionTopList {
    region: String,
    top_gifts: Vec<String>,
}

struct TopGift {
    gift_name: String,
}

#[get("/18/regions/top_list/{number}")]
async fn regions_top_list(
    pool: web::Data<PgPool>,
    number: web::Path<i64>,
) -> ShuttleResult<impl Responder> {
    let number = number.into_inner();

    let all_regions = sqlx::query_as!(Region, "SELECT id, name \"name!\" FROM regions")
        .fetch_all(pool.as_ref())
        .await?;

    let mut regions_top_list = Vec::new();
    for region in all_regions {
        let top_gifts = sqlx::query_as!(
            TopGift,
            "SELECT 
                gift_name \"gift_name!\"
            FROM 
                orders 
            WHERE 
                region_id = $1 
            GROUP BY 
                gift_name 
            ORDER BY 
                SUM(quantity) DESC, gift_name
            LIMIT $2",
            region.id,
            number
        )
        .fetch_all(pool.as_ref())
        .await?;

        regions_top_list.push(RegionTopList {
            region: region.name,
            top_gifts: top_gifts.into_iter().map(|gift| gift.gift_name).collect(),
        });
    }
    regions_top_list.sort_unstable_by_key(|r| r.region.clone());

    Ok(HttpResponse::Ok().json(regions_top_list))
}
