use actix_web::{get, web, HttpResponse, Responder};
use serde_json;

use crate::model::task::Task;

#[get("/")]
pub async fn get_tasks() -> impl Responder {
    let task = Task {
        id: 1,
        user_id: 2,
        project_id: 3,
        status: String::from("in progress"),
        description: String::from("Complete the project"),
    };

    let body = serde_json::to_string(&task).unwrap();
    HttpResponse::Ok().body(body)
}

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tasks);
}
