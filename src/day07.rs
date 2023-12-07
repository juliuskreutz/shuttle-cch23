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
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Ingredients {
    flour: usize,
    sugar: usize,
    butter: usize,
    #[serde(rename = "baking powder")]
    baking_powder: usize,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: usize,
}

#[get("/7/bake")]
async fn bake(request: HttpRequest) -> ShuttleResult<impl Responder> {
    let cookie = request.cookie("recipe").context("No recipe cookie")?;
    let encoded = cookie.value();

    let decoded = String::from_utf8(general_purpose::STANDARD.decode(encoded)?)?;
    let mut recipe: Recipe = serde_json::from_str(&decoded)?;

    let mut count = 0;
    loop {
        if recipe.recipe.flour > recipe.pantry.flour
            || recipe.recipe.sugar > recipe.pantry.sugar
            || recipe.recipe.butter > recipe.pantry.butter
            || recipe.recipe.baking_powder > recipe.pantry.baking_powder
            || recipe.recipe.chocolate_chips > recipe.pantry.chocolate_chips
        {
            break;
        }

        recipe.pantry.flour -= recipe.recipe.flour;
        recipe.pantry.sugar -= recipe.recipe.sugar;
        recipe.pantry.butter -= recipe.recipe.butter;
        recipe.pantry.baking_powder -= recipe.recipe.baking_powder;
        recipe.pantry.chocolate_chips -= recipe.recipe.chocolate_chips;

        count += 1;
    }

    Ok(HttpResponse::Ok().json(serde_json::json!(
        {
            "count": count,
            "pantry": recipe.pantry,
        }
    )))
}
