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

    let decoded = String::from_utf8(general_purpose::STANDARD.decode(encoded)?)?;

    Ok(HttpResponse::Ok().json(serde_json::from_str::<Value>(&decoded)?))
}

#[derive(serde::Deserialize)]
struct Recipe {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[get("/7/bake")]
async fn bake(request: HttpRequest) -> ShuttleResult<impl Responder> {
    let cookie = request.cookie("recipe").context("No recipe cookie")?;
    let encoded = cookie.value();

    let decoded = String::from_utf8(general_purpose::STANDARD.decode(encoded)?)?;
    let mut recipe: Recipe = serde_json::from_str(&decoded)?;

    let mut count = 0;
    'outer: loop {
        for (key, &value) in &recipe.recipe {
            if recipe.pantry.get(key).copied().unwrap_or_default() < value {
                break 'outer;
            }
        }

        for (key, &value) in &recipe.recipe {
            *recipe.pantry.get_mut(key).unwrap() -= value;
        }

        count += 1;
    }

    Ok(HttpResponse::Ok().json(serde_json::json!(
        {
            "count": count,
            "pantry": recipe.pantry,
        }
    )))
}
