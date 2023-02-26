use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub status: String,
    pub description: String,
}
