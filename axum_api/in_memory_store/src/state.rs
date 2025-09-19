use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct UserStore {
    pub id: String,
    pub name: String,
    pub email: String,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

pub type UserStorage = Arc<Mutex<HashMap<String, UserStore>>>;

#[derive(Clone)]
pub struct AppState {
    pub users: UserStorage,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
