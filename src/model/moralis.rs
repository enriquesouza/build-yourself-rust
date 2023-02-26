use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Moralis {
    pub KEY: String,
    pub DOMAIN: String,
    pub STATEMENT: String,
    pub URI: String,
    pub TIMEOUT: String,
}
