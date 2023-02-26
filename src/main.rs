// File: src/main.rs

use crate::model::secrets::{MongoDB, Moralis};
use actix_web::{App, HttpServer};
use aws::secrets::{get_mongodb_secret, get_moralis_secret};
use repository::mongodb::get_mongo_collection_names;

mod api;

mod model {
    pub mod secrets;
    pub mod user;
}

mod aws {
    pub mod s3;
    pub mod secrets;
}

mod repository {
    pub mod mongodb;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let moralis: Moralis = get_moralis_secret().await;
    println!("Moralis: {:?}", moralis.key);

    let mongodb_secret: MongoDB = get_mongodb_secret().await;
    println!("MongoDB: {:?}", mongodb_secret.url);

    let collections = get_mongo_collection_names().await;

    for collection in collections {
        println!("Collection: {:?}", collection);
    }

    HttpServer::new(|| App::new().service(api::api_scope()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
