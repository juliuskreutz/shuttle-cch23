use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(unsafe_html).service(safe_html);
}

#[derive(serde::Deserialize)]
struct Content {
    content: String,
}

fn format_html(content: &str) -> String {
    format!(
        "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        content
    )
}

#[post("/14/unsafe")]
async fn unsafe_html(html: web::Json<Content>) -> impl Responder {
    let html = format_html(&html.content);

    HttpResponse::Ok().body(html)
}

#[post("/14/safe")]
async fn safe_html(html: web::Json<Content>) -> impl Responder {
    let html = format_html(&html_escape::encode_quoted_attribute(&html.content));

    HttpResponse::Ok().body(html)
}
