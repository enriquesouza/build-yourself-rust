use bson::oid::ObjectId;
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNft {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub folder_name: String,
    pub file_name: String,
    pub erc20_address: String,
    pub user_nft_bundle_id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNftBundle {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
    pub erc20_address: String,
}
