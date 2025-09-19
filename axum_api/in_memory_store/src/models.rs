use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}
