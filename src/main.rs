// File: src/main.rs

use crate::model::moralis::Moralis;
use actix_web::{App, HttpServer};
use aws::secrets::get_moralis_secret;

mod api;

mod model {
    pub mod moralis;
    pub mod task;
}

mod aws {
    pub mod secrets;
    pub mod s3;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let moralis: Moralis = get_moralis_secret().await;
    println!("Moralis: {:?}", moralis.KEY);

    HttpServer::new(|| App::new().service(api::api_scope()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
