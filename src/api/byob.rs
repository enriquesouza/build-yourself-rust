use crate::aws;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json;

#[get("/s3-files")]
pub async fn get_s3_files() -> impl Responder {
    let result = aws::s3::show_s3_objects("byob-boop-dev").await;
    let body = serde_json::to_string(&result.unwrap()).unwrap();
    HttpResponse::Ok().body(body)
}

pub fn byob_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_s3_files);
}
