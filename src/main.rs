mod day00;
mod day01;
mod day04;
mod day05;
mod day06;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.configure(day00::configure)
            .configure(day01::configure)
            .configure(day04::configure)
            .configure(day05::configure)
            .configure(day06::configure);
    };

    Ok(config.into())
}
