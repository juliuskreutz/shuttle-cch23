use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};

use crate::ShuttleResult;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(weight).service(drop);
}

#[derive(serde::Deserialize)]
struct Pokemon {
    weight: f64,
}

#[get("/8/weight/{id}")]
async fn weight(path: web::Path<usize>) -> ShuttleResult<impl Responder> {
    let pokemon: Pokemon = reqwest::get(&format!("https://pokeapi.co/api/v2/pokemon/{}", path))
        .await?
        .json()
        .await?;

    Ok((pokemon.weight / 10.0).to_string())
}

#[get("/8/drop/{id}")]
async fn drop(path: web::Path<usize>) -> ShuttleResult<impl Responder> {
    let pokemon: Pokemon = reqwest::get(&format!("https://pokeapi.co/api/v2/pokemon/{}", path))
        .await?
        .json()
        .await?;

    let momentum = (pokemon.weight * (9.825f64 * 2.0 * 10.0).sqrt()) / 10.0;

    Ok(momentum.to_string())
}
