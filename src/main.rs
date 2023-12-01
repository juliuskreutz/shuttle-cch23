mod day00;
mod day01;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.configure(day00::configure).configure(day01::configure);
    };

    Ok(config.into())
}
