use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Moralis {
    pub key: String,
    pub domain: String,
    pub statement: String,
    pub uri: String,
    pub timeout: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct MongoDB {
    pub url: String,
}