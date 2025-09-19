use crate::handlers::{
    create_users, delete_user_by_id, get_all_users, get_user_by_id, hello, status,
    update_user_by_id,
};
use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // Basic routes
        .route("/", get(status))
        .route("/hello", get(hello))
        // User routes
        .route("/users", get(get_all_users))
        .route("/users", post(create_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/:id", put(update_user_by_id))
        .route("/users/:id", delete(delete_user_by_id))
        .with_state(state)
}
