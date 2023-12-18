mod day00;
mod day01;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;

use actix_web::web::{Data, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

type ShuttleResult<T> = Result<T, Box<dyn std::error::Error>>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(local_uri = "postgres://postgres:@localhost/shuttle")]
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.configure(day00::configure)
            .configure(day01::configure)
            .configure(day04::configure)
            .configure(day05::configure)
            .configure(day06::configure)
            .configure(day07::configure)
            .configure(day08::configure)
            .configure(day11::configure)
            .configure(day12::configure)
            .configure(day13::configure)
            .configure(day14::configure)
            .configure(day15::configure)
            .configure(day18::configure)
            .app_data(Data::new(pool.clone()));
    };

    Ok(config.into())
}
