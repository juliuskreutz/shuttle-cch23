use actix_web::{get, web::ServiceConfig, Responder};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[get("/5")]
async fn index() -> impl Responder {
    "Day05"
}
