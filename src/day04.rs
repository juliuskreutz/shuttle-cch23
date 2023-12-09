use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use ordered_float::OrderedFloat;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(strength).service(contest);
}

#[derive(Default, serde::Deserialize)]
#[serde(default)]
struct Reindeer {
    name: String,
    strength: usize,
    speed: OrderedFloat<f64>,
    height: usize,
    antler_width: usize,
    snow_magic_power: usize,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten: usize,
}

#[post("/4/strength")]
async fn strength(reindeer: web::Json<Vec<Reindeer>>) -> impl Responder {
    reindeer
        .iter()
        .map(|r| r.strength)
        .sum::<usize>()
        .to_string()
}

#[post("/4/contest")]
async fn contest(reindeer: web::Json<Vec<Reindeer>>) -> impl Responder {
    let fastest = reindeer.iter().max_by_key(|r| r.speed).unwrap();
    let tallest = reindeer.iter().max_by_key(|r| r.height).unwrap();
    let magician = reindeer.iter().max_by_key(|r| r.snow_magic_power).unwrap();
    let consumer = reindeer.iter().max_by_key(|r| r.candies_eaten).unwrap();

    let fastest_text = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest.strength, fastest.name
    );
    let tallest_text = format!(
        "{} is standing tall with his {} cm wide antlers",
        tallest.name, tallest.antler_width
    );
    let magician_text = format!(
        "{} could blast you away with a snow magic power of {}",
        magician.name, magician.snow_magic_power
    );
    let consumer_text = format!(
        "{} ate lots of candies, but also some {}",
        consumer.name, consumer.favorite_food
    );

    HttpResponse::Ok().json(serde_json::json!({
        "fastest": fastest_text,
        "tallest": tallest_text,
        "magician": magician_text,
        "consumer": consumer_text,
    }))
}
