use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(hello_world).service(error);
}

#[get("/")]
async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[get("/-1/error")]
async fn error() -> impl Responder {
    HttpResponse::InternalServerError()
}
