use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use fancy_regex::Regex;
use reqwest::StatusCode;
use sha2::{Digest, Sha256};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(nice).service(game);
}

#[derive(serde::Deserialize)]
struct Input {
    input: String,
}

#[post("/15/nice")]
async fn nice(value: Option<web::Json<Input>>) -> impl Responder {
    let Some(input) = value.map(|v| v.into_inner().input.to_lowercase()) else {
        return HttpResponse::BadRequest().finish();
    };

    let mut nice = true;

    let re = Regex::new(r"[aeiouy].*[aeiouy].*[aeiouy]").unwrap();
    nice &= re.is_match(&input).unwrap();

    let re = Regex::new(r"([a-z])\1").unwrap();
    nice &= re.is_match(&input).unwrap();

    let re = Regex::new(r"ab|cd|pq|xy").unwrap();
    nice &= !re.is_match(&input).unwrap();

    if nice {
        HttpResponse::Ok().json(serde_json::json!({"result": "nice"}))
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({"result": "naughty"}))
    }
}

#[post("/15/game")]
async fn game(value: Option<web::Json<Input>>) -> impl Responder {
    let Some(input) = value.map(|v| v.into_inner().input) else {
        return HttpResponse::BadRequest().finish();
    };

    type Rule = fn(&str) -> bool;
    let rules: Vec<(Rule, _)> = vec![
        (
            |input| input.len() >= 8,
            (StatusCode::BAD_REQUEST, "8 chars"),
        ),
        (
            |input| {
                input.chars().any(|c| c.is_uppercase())
                    && input.chars().any(|c| c.is_lowercase())
                    && input.chars().any(|c| c.is_ascii_digit())
            },
            (StatusCode::BAD_REQUEST, "more types of chars"),
        ),
        (
            |input| input.chars().filter(|c| c.is_ascii_digit()).count() >= 5,
            (StatusCode::BAD_REQUEST, "55555"),
        ),
        (
            |input| {
                let re = Regex::new(r"\d+").unwrap();
                re.find_iter(input)
                    .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
                    .sum::<u32>()
                    == 2023
            },
            (StatusCode::BAD_REQUEST, "math is hard"),
        ),
        (
            |input| {
                let re = Regex::new(r"j.*o.*y").unwrap();
                re.is_match(input).unwrap()
                    && input
                        .chars()
                        .filter(|c| matches!(c, 'j' | 'o' | 'y'))
                        .count()
                        == 3
            },
            (StatusCode::NOT_ACCEPTABLE, "not joyful enough"),
        ),
        (
            |input| {
                let re = Regex::new(r"([a-zA-Z]).\1").unwrap();
                re.is_match(input).unwrap()
            },
            (
                StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                "illegal: no sandwich",
            ),
        ),
        (
            |input| input.chars().any(|c| matches!(c, '\u{2980}'..='\u{2BFF}')),
            (StatusCode::RANGE_NOT_SATISFIABLE, "outranged"),
        ),
        (
            |input| input.chars().any(unic_emoji_char::is_emoji_presentation),
            (StatusCode::UPGRADE_REQUIRED, "😳"),
        ),
        (
            |input| {
                let mut hasher = Sha256::new();
                hasher.update(input);
                let hash = hasher.finalize();
                let hash = hex::encode(hash);

                hash.ends_with('a')
            },
            (StatusCode::IM_A_TEAPOT, "not a coffee brewer"),
        ),
    ];

    for (rule, (status, reason)) in rules {
        if !rule(&input) {
            return HttpResponse::build(status)
                .json(serde_json::json!({"result": "naughty", "reason": reason}));
        }
    }

    HttpResponse::Ok()
        .json(serde_json::json!({"result": "nice", "reason": "that's a nice password"}))
}
