use crate::model::moralis::Moralis;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_secretsmanager::{Client, Error};

pub async fn get_secret_by_name(secret_name: &str) -> Result<String, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let secret_id = format!("DEV/BYOB/{}", secret_name);

    // List secrets
    // let response = client.list_secrets().send().await?;

    let moralis_secret_response = client
        .get_secret_value()
        .secret_id(secret_id)
        .send()
        .await?;

    Ok(moralis_secret_response.secret_string().unwrap().to_string())
}

pub async fn get_generic_secret_by_name<T: serde::de::DeserializeOwned>(
    secret_name: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let secret_response = get_secret_by_name(secret_name).await?;
    let generic_response: T = serde_json::from_str(&secret_response)?;
    Ok(generic_response)
}

pub async fn get_moralis_secret() -> Moralis {
    get_generic_secret_by_name::<Moralis>("MORALIS")
        .await
        .unwrap()
}
