use std::collections::HashMap;

use actix_web::{get, web::ServiceConfig, HttpRequest, HttpResponse, Responder};
use anyhow::Context;
use base64::{engine::general_purpose, Engine};
use serde_json::Value;

use crate::ShuttleResult;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(decode).service(bake);
}

#[get("/7/decode")]
async fn decode(request: HttpRequest) -> ShuttleResult<impl Responder> {
    let cookie = request.cookie("recipe").context("No recipe cookie")?;
    let encoded = cookie.value();

    let decoded = general_purpose::STANDARD.decode(encoded)?;

    Ok(HttpResponse::Ok().json(serde_json::from_slice::<Value>(&decoded)?))
}

#[derive(serde::Deserialize)]
struct Bake {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[get("/7/bake")]
async fn bake(request: HttpRequest) -> ShuttleResult<impl Responder> {
    let cookie = request.cookie("recipe").context("No recipe cookie")?;
    let encoded = cookie.value();

    let decoded = general_purpose::STANDARD.decode(encoded)?;
    let mut bake: Bake = serde_json::from_slice(&decoded)?;

    let cookies = bake
        .recipe
        .iter()
        .map(|(k, &v)| {
            if v == 0 {
                usize::MAX
            } else {
                bake.pantry.get(k).map(|&a| a / v).unwrap_or_default()
            }
        })
        .min()
        .unwrap_or_default();

    for (key, &value) in &bake.recipe {
        if let Some(p) = bake.pantry.get_mut(key) {
            *p -= cookies * value;
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!(
        {
            "cookies": cookies,
            "pantry": bake.pantry,
        }
    )))
}
