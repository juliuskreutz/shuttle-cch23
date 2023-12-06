use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[post("/6")]
async fn index(request: web::Bytes) -> impl Responder {
    let s = String::from_utf8(request.to_vec()).unwrap();

    let elf = s.matches("elf").count();
    let elf_on_shelf = s.matches("elf on a shelf").count();
    let elf_no_shelf = s.matches("shelf").count() - elf_on_shelf;

    HttpResponse::Ok().json(serde_json::json!({
        "elf": elf,
        "elf on a shelf": elf_on_shelf,
        "shelf with no elf on it": elf_no_shelf
    }))
}
