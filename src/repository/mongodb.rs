use mongodb::{Client, Collection};

use crate::{aws::secrets::get_mongodb_secret, model::user::User};

pub async fn get_mongo_client() -> Client {
    let mongo_db_url = get_mongodb_secret().await;

    let client_options = mongodb::options::ClientOptions::parse(mongo_db_url.url.to_string())
        .await
        .unwrap();
    Client::with_options(client_options).unwrap()
}

pub async fn get_mongo_database() -> mongodb::Database {
    let client = get_mongo_client().await;
    client.database("byob")
}

pub async fn get_mongo_collection_by_name() -> Collection<User> {
    get_mongo_database().await.collection("users")
}

pub async fn get_mongo_collection_names() -> Vec<String> {
    let client = get_mongo_client().await;
    client
        .database("byob")
        .list_collection_names(None)
        .await
        .unwrap()
}
