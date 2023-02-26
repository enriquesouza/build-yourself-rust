use actix_web::{web, Scope};
pub mod user;

pub mod byob;

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(web::scope("/users").configure(user::user_routes))
        .service(web::scope("/byob").configure(byob::byob_routes))
}
