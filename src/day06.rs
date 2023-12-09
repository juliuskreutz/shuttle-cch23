use actix_web::{post, web::ServiceConfig, HttpResponse, Responder};
use fancy_regex::Regex;

use crate::ShuttleResult;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[post("/6")]
async fn index(s: String) -> ShuttleResult<impl Responder> {
    let elf = s.matches("elf").count();
    let elf_on_shelf = Regex::new(r"(?=(elf on a shelf))")?.find_iter(&s).count();
    let elf_no_shelf = s.matches("shelf").count() - elf_on_shelf;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "elf": elf,
        "elf on a shelf": elf_on_shelf,
        "shelf with no elf on it": elf_no_shelf
    })))
}
