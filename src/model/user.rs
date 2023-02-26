use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub created_at: NaiveDateTime,
    pub reset_password_token: Option<String>,
    pub reset_password_expires: Option<NaiveDateTime>,
    pub primary_erc20_address: Option<String>,
    pub default_erc20_address: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Admin,
    Company,
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}
