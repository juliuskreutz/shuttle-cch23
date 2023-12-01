use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(cube_the_bits);
}

#[get("/1/{slug:(\\d+/?)+}")]
async fn cube_the_bits(path: web::Path<String>) -> impl Responder {
    path.into_inner()
        .split('/')
        .flat_map(str::parse::<i32>)
        .reduce(|a, b| a ^ b)
        .unwrap()
        .pow(3)
        .to_string()
}
