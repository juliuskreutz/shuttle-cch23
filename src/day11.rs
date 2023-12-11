use std::io::BufReader;

use actix_files::NamedFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web::ServiceConfig, Responder};
use image::{GenericImageView, Rgba};

use crate::ShuttleResult;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(decoration).service(red_pixels);
}

#[get("/11/assets/decoration.png")]
async fn decoration() -> ShuttleResult<impl Responder> {
    Ok(NamedFile::open("assets/decoration.png")?)
}

#[derive(MultipartForm)]
struct File {
    image: TempFile,
}

#[post("/11/red_pixels")]
async fn red_pixels(MultipartForm(file): MultipartForm<File>) -> ShuttleResult<impl Responder> {
    let reader = BufReader::new(&file.image.file);
    let image = image::load(reader, image::ImageFormat::Png)?;

    let mut count = 0;
    for (_, _, Rgba([r, g, b, _])) in image.pixels() {
        if r > b.saturating_add(g) {
            count += 1;
        }
    }

    Ok(count.to_string())
}
