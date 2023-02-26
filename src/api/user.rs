// src/api/user.rs
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/list")]
pub async fn list_users() -> impl Responder {
    HttpResponse::Ok().body("List of users")
}

#[post("/create")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Created().body("User created")
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users);
    cfg.service(create_user);
}