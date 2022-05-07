use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ABC {
    login: String,
    id: i32,
    #[serde(rename = "avatar_url")]
    test_id: String,
}