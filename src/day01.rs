use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(one_two_three);
}

#[get("/1/{slug:(\\d+/?)+}")]
async fn one_two_three(path: web::Path<String>) -> impl Responder {
    let slug = path.into_inner();

    let nums = slug
        .split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut xor = nums[0];

    for num in nums.iter().skip(1) {
        xor ^= num;
    }

    (xor * xor * xor).to_string()
}
